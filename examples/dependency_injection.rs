trait EmailSender {
    fn send_email(&self, to: &str);
}

fn send_to_gmail(email_sender: impl EmailSender, alias: &str) {
    email_sender.send_email(&format!("{alias}@gmail.com"));
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_send_gmail() {
        struct MockEmailSender(sneaky::Spy<String>);
        impl EmailSender for MockEmailSender {
            fn send_email(&self, to: &str) {
                self.0.call(to.to_string());
            }
        }

        // Given
        let (send_email, spy) = sneaky::spy();
        let email_sender = MockEmailSender(spy);

        // When
        send_to_gmail(email_sender, "bob");

        // Then
        send_email
            .called_with("bob@gmail.com".to_string())
            .called_no_more();
    }
}
