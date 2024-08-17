use ch15_05_refcell_pointer::*;
use std::cell::RefCell;

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    // Hay que usar RefCell ya que no podemos usar una referencia mutable aquí
    // Messenger::send(&self, msf: &str)
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }

    // No puede haber dos referencias mutables a la vez
    // Al comprobar las borrowing rules en ejecución, hay un  
    fn send_more(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }

}

#[test]
fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
}

#[test]
fn it_sends_more() {
    let mock_messenger: MockMessenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_values(80);

    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
}
