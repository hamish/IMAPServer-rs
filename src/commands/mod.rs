use tokio::io::AsyncWriteExt;
use tokio::net::tcp::split::WriteHalf;

pub async fn capability(
    args: Vec<&str>,
    write: &mut WriteHalf<'_>,
) {
    let identifier = args[0];

    write.write_all(b"* CAPABILITY IMAP4rev1 AUTH=PLAIN UTF8=ACCEPT LOGINDISABLED\r\n").await
        .expect("failed to write data to socket");
    write.write_all(format!("{}{}", identifier, " OK CAPABILITY completed\r\n").as_ref())
        .await
        .expect("failed to write data to socket");

    //Print to view for debug
    debug!(
        "{}",
        "* CAPABILITY IMAP4rev1 AUTH=PLAIN UTF8=ACCEPT LOGINDISABLED\r\n"
    );
    debug!("{}{}", identifier, " OK CAPABILITY completed");
}

pub async fn list(
    args: Vec<&str>,
    write: &mut WriteHalf<'_>,
) {
    let identifier = args[0];

    write.write_all(b"* LIST () \"/\" INBOX\r\n").await
        .expect("failed to write data to socket");
    write.write_all(format!("{}{}", identifier, " OK LIST Completed\r\n").as_ref())
        .await
        .expect("failed to write data to socket");

    //Print to view for debug
    debug!("{}", "* LIST () \"/\" \"INBOX\"\r\n");
    debug!("{}{}", identifier, " OK LIST Completed\r\n");
}

pub async fn uid(
    args: Vec<&str>,
    write: &mut WriteHalf<'_>,
) {
    let identifier = args[0];

    write.write_all(b"* 1 FETCH (FLAGS (\\Seen) UID 1)\r\n")
        .await
        .expect("failed to write data to socket");
    write.write_all(format!("{}{}", identifier, " OK UID FETCH completed\r\n").as_ref())
        .await
        .expect("failed to write data to socket");

    //Print to view for debug
    debug!("{}", "* 1 FETCH (FLAGS (\\Seen) UID 1)\r\n");
    debug!("{}{}", identifier, " OK UID FETCH completed\r\n");
}

pub async fn logout(
    args: Vec<&str>,
    write: &mut WriteHalf<'_>,
) {
    let identifier = args[0];

    write.write_all(b"* BYE IMAP4rev1 Server logging out\r\n")
        .await
        .expect("failed to write data to socket");
    write.write_all(format!("{}{}", identifier, " OK LOGOUT completed").as_ref())
        .await
        .expect("failed to write data to socket");

    //Print to view for debug
    debug!("{}", "* BYE IMAP4rev1 Server logging out\r\n");
    debug!("{}{}", identifier, " OK LOGOUT completed\r\n");
}

pub mod authenticate;

#[deprecated(since = "0.0.1", note = "please use `commands::authenticate::authenticate` instead")]
pub async fn login(
    args: Vec<&str>,
    write: &mut WriteHalf<'_>,
) {
    let identifier = args[0];

    write.write_all(format!("{} {}", identifier, "OK LOGIN completed\r\n").as_ref())
        .await
        .expect("failed to write data to socket");

    //Print to view for debug
    debug!("{} {}", identifier, "OK LOGIN completed\r\n");
}

pub async fn noop(
    args: Vec<&str>,
    write: &mut WriteHalf<'_>,
) {
    let identifier = args[0];

    write.write_all(format!("{} {}", identifier, "OK NOOP completed\r\n").as_ref())
        .await
        .expect("failed to write data to socket");

    //Print to view for debug
    debug!("{} {}", identifier, "OK NOOP completed");
}

pub async fn select(
    args: Vec<&str>,
    write: &mut WriteHalf<'_>,
) {
    let identifier = args[0];

    write.write_all(b"* 1 EXISTS\r\n").await
        .expect("failed to write data to socket");
    write.write_all(b"* 0 RECENT\r\n").await
        .expect("failed to write data to socket");
    write.write_all(b"* OK [UNSEEN 1] Message 1 is first unseen\r\n").await
        .expect("failed to write data to socket");
    write.write_all(b"* OK [UIDNEXT 1] Predicted next UID\r\n")
        .await
        .expect("failed to write data to socket");
    write.write_all(b"* FLAGS (\\Answered \\Flagged \\Deleted \\Seen \\Draft)\r\n").await
        .expect("failed to write data to socket");
    write.write_all(b"* OK [PERMANENTFLAGS (\\Deleted \\Seen \\*)] Limited\r\n").await
        .expect("failed to write data to socket");
    write.write_all(format!(
        "{} {}",
        identifier,
        "OK [READ-WRITE] SELECT completed\r\n"
    ).as_ref()).await
        .expect("failed to write data to socket");

    //Print to view for debug
    debug!("{} {}", identifier, "OK [READ-WRITE] SELECT completed");
}

pub async fn check(
    args: Vec<&str>,
    write: &mut WriteHalf<'_>,
) {
    let identifier = args[0];
    write.write_all(format!("{} {}", identifier, "OK CHECK Completed\r\n").as_ref())
        .await
        .expect("failed to write data to socket");

    //Print to view for debug
    debug!("{} {}", identifier, "OK CHECK Completed");
}
