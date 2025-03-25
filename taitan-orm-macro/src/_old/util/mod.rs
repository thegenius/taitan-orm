mod parser;
mod utils;
mod life_time_checker;
mod copy_struct;

pub use utils::extract_fields;
pub use utils::create_path_from_str;
pub use life_time_checker::extract_generic_lifetimes;
pub use life_time_checker::build_impl_trait_token;
pub use copy_struct::copy_to_template_struct;