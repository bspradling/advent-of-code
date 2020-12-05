#[test]
fn test_ticket_parsing() {
    let ticket = BoardingTicket::from("FBFFBFFRLL");

    assert_eq!(ticket.row(), "FBFFBFF");
    assert_eq!(ticket.column(), "RLL");
}

#[test]
fn test_seat_id_computation() {
    let first_ticket = BoardingTicket::from("FBFBBFFRLR");
    let second_ticket = BoardingTicket::from("BFFFBBFRRR");
    let third_ticket = BoardingTicket::from("FFFBBBFRRR");
    let fourth_ticket = BoardingTicket::from("BBFFBBFRLL");

    assert_eq!(first_ticket.seat_id(), 357);
    assert_eq!(second_ticket.seat_id(), 567);
    assert_eq!(third_ticket.seat_id(), 119);
    assert_eq!(fourth_ticket.seat_id(), 820);
}
