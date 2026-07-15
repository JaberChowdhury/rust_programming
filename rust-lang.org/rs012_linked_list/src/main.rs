use crate::LinkedList::*;

enum LinkedList {
    Node(i32, Box<LinkedList>),
    End,
}

impl LinkedList {
    fn create_empty() -> LinkedList {
        End
    }

    fn add_front(self, new_data: i32) -> LinkedList {
        Node(new_data, Box::new(self))
    }

    fn count_length(&self) -> i32 {
        match *self {
            Node(_, ref next_node) => 1 + next_node.count_length(),
            End => 0,
        }
    }

    fn print_format(&self) -> String {
        match *self {
            Node(current_data, ref next_node) => {
                format!("{}, {}", current_data, next_node.print_format())
            }
            End => {
                format!("End")
            }
        }
    }
}

fn main() {
    let mut my_list = LinkedList::create_empty();

    my_list = my_list.add_front(32441);
    my_list = my_list.add_front(221314);
    my_list = my_list.add_front(3);

    println!("Total length of linked list: {}", my_list.count_length());
    println!("Values of linked list {}", my_list.print_format());
}
