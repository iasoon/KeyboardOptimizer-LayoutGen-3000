pub struct AssignmentTable<'a, T> {
    kb_def: &'a KbDef,
    assignment_map: &'a Table<Assignment, Option<Num<AllowedAssignment>>>,
    values: Table<AllowedAssignment, T>,
}

impl<'a, T> AssignmentTable<'a, T> {
    fn new(kb_def: &'a KbDef)
}


impl<'a, T> Dict<Assignment, T> for AssignmentTable<'a, T> {
    
}
