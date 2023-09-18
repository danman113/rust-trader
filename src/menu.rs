use dialoguer::{console::Term, Select};

pub type MenuItem<'a, Arg> = (&'a str, Box<dyn FnMut(Arg)>);

pub fn prompt_menu<'a, Arg>(term: &Term, prompt: &str, mut menu: Vec<MenuItem<'a, Arg>>, arg: Arg) {
    let items: Vec<&str> = menu.iter().map(|item| item.0).collect();
    let choice = Select::new()
        .with_prompt(prompt)
        .items(&items)
        .default(0)
        .interact_on(term)
        .expect("Chose invalid option");
    let (_, boxed_fn) = menu.get_mut(choice).unwrap();
    boxed_fn(arg);
}
