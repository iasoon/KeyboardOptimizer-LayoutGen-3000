use cat::*;
use data::*;

pub trait Assignable {
    fn assign(&mut self, kb_def: &KbDef, assignment: Assignment) {
        match assignment {
            Assignment::Free { free_num, loc_num } => {
                let &token_num = kb_def.frees.get(free_num);
                self.assign_token(token_num, loc_num);
            },
            Assignment::Lock { lock_num, key_num } => {
                let lock_entries = kb_def.locks.get(lock_num);
                for (layer_num, &value) in lock_entries.enumerate() {
                    if let Some(token_num) = value {
                        self.assign_token(token_num, kb_def.loc_num().apply(Loc {
                            key_num: key_num,
                            layer_num: layer_num,
                        }));
                    }
                }
            }
        }
    }

    fn assign_token(&mut self, token_num: Num<Token>, loc_num: Num<Loc>);
}
