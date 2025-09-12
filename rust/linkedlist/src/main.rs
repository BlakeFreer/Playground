use linkedlist::bad_stack_struct;

fn main() {
    {
        let list = bad_stack_struct::List::new(vec![1, 2, 3, 4]);
        println!("{list}");
    }
}
