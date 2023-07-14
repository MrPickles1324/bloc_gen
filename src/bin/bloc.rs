use std::{env, fs::File, io::Write, process::exit};

use bloc_gen::util::uppercase_first_letter;

fn main() -> anyhow::Result<()> {
    let name = env::args().nth(1).unwrap_or_else(|| {
        println!("Please specify bloc name");
        exit(1);
    });
    let capitalized_name = uppercase_first_letter(name.as_str());

    println!("creating {name}_bloc.dart");
    let mut bloc_file = File::create(format!("{name}_bloc.dart"))?;
    bloc_file.write_fmt(format_args!(
        "import 'package:bloc/bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

part '{name}_bloc.freezed.dart';
part '{name}_state.dart';
part '{name}_event.dart';

class {capitalized_name}Bloc extends Bloc<{capitalized_name}Event, {capitalized_name}State> {{
  {capitalized_name}Bloc() : super(const {capitalized_name}State.initial());
}}"
    ))?;

    println!("creating {name}_state.dart");
    let mut state_file = File::create(format!("{name}_state.dart"))?;
    state_file.write_fmt(format_args!(
        "part of '{name}_bloc.dart';

@freezed
class {capitalized_name}State with _${capitalized_name}State {{
  const factory {capitalized_name}State.initial() = _Initial;
}}"
    ))?;

    println!("creating {name}_event.dart");
    let mut event_file = File::create(format!("{name}_event.dart"))?;
    event_file.write_fmt(format_args!(
        "part of '{name}_bloc.dart';

@freezed
class {capitalized_name}Event with _${capitalized_name}Event {{
  const factory {capitalized_name}Event.started() = _Started;
}}"
    ))?;
    Ok(())
}
