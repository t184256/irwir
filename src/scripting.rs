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

use actions::MapToKey;

pub type IrwirGluonFunc<'t> = FunctionRef<'t, fn(i32) -> MapToKey>;

pub struct ScriptingEngine {
    vm: RootedThread,
}

impl ScriptingEngine {
    pub fn new() -> Self {
        let vm = new_vm();
        vm.register_type::<MapToKey>("MapToKey", &[]).unwrap();
        fn load_mod(vm: &gluon::Thread) -> gluon::vm::Result<ExternModule> {
            ExternModule::new(
                vm,
                record! {
                    map_to_key => primitive!(1 MapToKey::new),
                },
            )
        }
        add_extern_module(&vm, "irwir_internals", load_mod);
        ScriptingEngine { vm }
    }

    pub fn make_func(&self, s: &String) -> IrwirGluonFunc {
        let prelude = indoc!(
            r#"
            let {map_to_key} = import! irwir_internals
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
        let mut f = se.make_func("map_to_key \"Whoa\"".to_string());
        let a = f.call(3).unwrap();
        assert_eq!(a, MapToKey::new("Whoa".to_string()));
    }
}