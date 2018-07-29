// Copyright (c) 2018 Alexander Sosedkin <monk@unboiled.info>

// irwir is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// irwir is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with irwir.  If not, see <http://www.gnu.org/licenses/>.

use gluon;
use gluon::import::add_extern_module;
use gluon::vm::api::FunctionRef;
use gluon::vm::ExternModule;
use gluon::{new_vm, Compiler, RootedThread};

use actions::Action;

pub type IrwirGluonFunc<'t> = FunctionRef<'t, fn(i32) -> Action>;

pub struct ScriptingEngine {
    vm: RootedThread,
}

impl ScriptingEngine {
    pub fn new() -> Self {
        let vm = new_vm();
        vm.register_type::<Action>("Action", &[]).unwrap();
        fn load_mod(vm: &gluon::Thread) -> gluon::vm::Result<ExternModule> {
            ExternModule::new(
                vm,
                record! {
                    combo => primitive!(1 |actions| Action::Combo(actions)),
                    nothing => Action::Nothing,
                    event => primitive!(3 |kind_name, code_name, value|
                                           Action::Event{kind_name, code_name, value}),
                    syn => Action::Syn,
                    key => primitive!(1 |key_name| Action::Key(key_name)),
                    button => primitive!(1 |key_name| Action::Button(key_name)),
                    press => primitive!(1 |key_name| Action::Press(key_name)),
                    release => primitive!(1 |key_name| Action::Release(key_name)),
                    repeat => primitive!(1 |key_name| Action::Repeat(key_name)),
                    press_release => primitive!(1 |key_name| Action::PressRelease(key_name)),
                    abs => primitive!(2 |abs_name, value| Action::Abs(abs_name, value)),
                    rel => primitive!(2 |rel_name, value| Action::Rel(rel_name, value)),
                    xy => primitive!(2 |value_x, value_y| Action::XY(value_x, value_y)),
                },
            )
        }
        add_extern_module(&vm, "irwir_internals", load_mod);
        ScriptingEngine { vm }
    }

    pub fn make_func(&self, s: &String) -> IrwirGluonFunc {
        let prelude = indoc!(
            r#"
            let { combo, nothing, event, syn,
                  button, key, press, release, repeat, press_release,
                  abs, rel, xy } = import! irwir_internals
            \value ->
        "#
        );
        let code = format!("{} (\n{})", prelude, s);
        Compiler::new()
            .run_expr::<IrwirGluonFunc>(&self.vm, "", code.as_str())
            .unwrap()
            .0
    }
}

#[cfg(test)]
mod tests {
    use scripting::*;
    #[test]
    fn scripting_test() {
        let se = ScriptingEngine::new();
        let mut f = se.make_func(&"key \"KeyA\"".to_string());
        let action: Action = f.call(3).unwrap();
        match action {
            Action::Key(key_action) => {
                assert_eq!(key_action, "KeyA");
            }
            _ => {
                unreachable!();
            }
        }
    }
}
