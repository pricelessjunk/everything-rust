/// macros are a way of writing code that writes other code, which is known as metaprogramming.

/* Declarative Macros */
// Uses `macro_rules!` The vec! macro looks like this

#[macro_export] // This attribute makes the macro available to other crates when you declare it in a library.
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/* Procedural Macros */

// See https://doc.rust-lang.org/book/ch20-05-macros.html#how-to-write-a-custom-derive-macro
// derive macros can be implemeted for any trait.

/* Attribute like Macros */
// same as custom derive macros, but can be applied to structs, enums, functions etc.

// #[route(GET, "/")]
// fn index() {}

// The definition
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

/* Function like Macros */

// Example let sql = sql!(SELECT * FROM posts WHERE id=1);

// The definitio
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {}
