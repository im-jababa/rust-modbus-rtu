use super::*;


#[test]
fn test_from_id() {
    assert_eq!(Baudrate::from_id(0), Some(Baudrate::BR1200));
    assert_eq!(Baudrate::from_id(1), Some(Baudrate::BR2400));
    assert_eq!(Baudrate::from_id(2), Some(Baudrate::BR4800));
    assert_eq!(Baudrate::from_id(3), Some(Baudrate::BR9600));
    assert_eq!(Baudrate::from_id(4), Some(Baudrate::BR19200));
    assert_eq!(Baudrate::from_id(5), Some(Baudrate::BR38400));
    assert_eq!(Baudrate::from_id(6), Some(Baudrate::BR57600));
    assert_eq!(Baudrate::from_id(7), Some(Baudrate::BR115200));
    assert_eq!(Baudrate::from_id(99), None);
}


#[test]
fn test_to_id() {
    assert_eq!(Baudrate::BR1200.to_id(), 0);
    assert_eq!(Baudrate::BR2400.to_id(), 1);
    assert_eq!(Baudrate::BR4800.to_id(), 2);
    assert_eq!(Baudrate::BR9600.to_id(), 3);
    assert_eq!(Baudrate::BR19200.to_id(), 4);
    assert_eq!(Baudrate::BR38400.to_id(), 5);
    assert_eq!(Baudrate::BR57600.to_id(), 6);
    assert_eq!(Baudrate::BR115200.to_id(), 7);
}