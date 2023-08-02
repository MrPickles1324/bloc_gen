use std::{env, fs::File, io::Write, process::exit};

use bloc_gen::util::uppercase_first_letter;

fn main() -> anyhow::Result<()> {
    let name = env::args().nth(1).unwrap_or_else(|| {
        println!("Please specify cubit name");
        exit(1);
    });

    let capitalized_name = uppercase_first_letter(name.as_str());

    println!("creating {name}_cubt.dart");
    let mut cubit_file = File::create(format!("{name}_cubit.dart"))?;
    cubit_file.write_fmt(format_args!(
        "import 'package:bloc/bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

part '{name}_bloc.freezed.dart';
part '{name}_state.dart';
part '{name}_event.dart';

class {capitalized_name}Cubit extends Cubit<{capitalized_name}State> {{
  {capitalized_name}Cubit() : super(const {capitalized_name}State.initial());
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
    Ok(())
}
