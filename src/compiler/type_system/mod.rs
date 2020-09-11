mod check;
mod infer;

///
/// ## Infer Types
///
/// - Fill in type information for all nodes
///
pub struct Infer {}

impl Infer
{
    pub fn new() -> Infer
    {
        Infer {}
    }
}

///
/// ## Check Types
/// 
/// - Verify that all inferred and annotated type information makes sense
/// 
pub struct Check {}

impl Check
{
    pub fn new() -> Check
    {
        Check {}
    }
}