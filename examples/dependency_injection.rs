trait EmailSender {
    fn send_email(&self, email_address: &str) -> Result<(), ()>;
}

fn send_email_if_custom_domain(email_sender: impl EmailSender, email_address: &str) -> Result<(), ()> {
    if email_address.ends_with("@gmail.com") {
        return Ok(());
    }

    email_sender.send_email(email_address)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_send_email_when_custom_domain() {
        struct MockEmailSender(verify_call::Caller<String>);
        impl EmailSender for MockEmailSender {
            fn send_email(&self, email_address: &str) -> Result<(), ()> {
                self.0.call(email_address.to_string());
                Ok(())
            }
        }

        // Given
        let (send_email, send_email_call) = verify_call::pair();
        let email_sender = MockEmailSender(send_email_call);

        // When
        let result = send_email_if_custom_domain(email_sender, "dennis@krasnov.dev");

        // Then
        assert!(result.is_ok());
        assert_eq!(send_email.calls(), &["dennis@krasnov.dev"]);
    }

    #[test]
    fn should_fail_when_send_email_fails() {
        struct MockEmailSender(verify_call::Caller<String>);
        impl EmailSender for MockEmailSender {
            fn send_email(&self, email_address: &str) -> Result<(), ()> {
                self.0.call(email_address.to_string());
                Err(())
            }
        }

        // Given
        let (send_email, send_email_call) = verify_call::pair();
        let email_sender = MockEmailSender(send_email_call);

        // When
        let result = send_email_if_custom_domain(email_sender, "dennis@krasnov.dev");

        // Then
        assert!(result.is_err());
        assert_eq!(send_email.calls(), &["dennis@krasnov.dev"]);
    }

    #[test]
    fn should_not_send_email_when_gmail_domain() {
        struct MockEmailSender(verify_call::Caller<String>);
        impl EmailSender for MockEmailSender {
            fn send_email(&self, email_address: &str) -> Result<(), ()> {
                self.0.call(email_address.to_string());
                Ok(())
            }
        }

        // Given
        let (send_email, send_email_call) = verify_call::pair();
        let email_sender = MockEmailSender(send_email_call);

        // When
        let result = send_email_if_custom_domain(email_sender, "dennis@gmail.com");

        // Then
        assert!(result.is_ok());
        assert_eq!(send_email.calls().len(), 0);
    }
}
