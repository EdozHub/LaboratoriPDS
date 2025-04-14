use es3lab3::CircularBuffer;

#[test]
pub fn test_new_buffer(){
    let buf:CircularBuffer<i32> = CircularBuffer::new(10);
    assert_eq!(buf.size,10);
    assert_eq!(buf.head,0);
    assert_eq!(buf.tail,0);
    assert!(buf.buffer.iter().all(|x| x.is_none()));
}

#[test]
pub fn test_write() {
    let mut buf: CircularBuffer<i32> = CircularBuffer::new(3);

    buf.write(Some(1)).unwrap();
    assert_eq!(buf.head, 0);
    assert_eq!(buf.tail, 1);

    buf.write(Some(2)).unwrap();
    assert_eq!(buf.head, 0);
    assert_eq!(buf.tail, 2);

    buf.write(Some(3)).unwrap();
    assert_eq!(buf.head, 0);
    assert_eq!(buf.tail, 0);

    let result = buf.write(Some(4));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Circular buffer is full");
}

#[test]
pub fn test_read() {
    let mut buf: CircularBuffer<i32> = CircularBuffer::new(3);

    buf.write(Some(1)).unwrap();
    buf.write(Some(2)).unwrap();
    buf.write(Some(3)).unwrap();

    assert_eq!(buf.read(), Some(1));
    assert_eq!(buf.head, 1);
    assert_eq!(buf.tail, 0);

    assert_eq!(buf.read(), Some(2));
    assert_eq!(buf.head, 2);
    assert_eq!(buf.tail, 0);

    assert_eq!(buf.read(), Some(3));
    assert_eq!(buf.head, 0);
    assert_eq!(buf.tail, 0);

    assert_eq!(buf.read(), None);
}

#[test]
pub fn test_clean(){
    let mut buf: CircularBuffer<i32> = CircularBuffer::new(3);
    buf.write(Some(1)).unwrap();
    buf.write(Some(2)).unwrap();
    buf.write(Some(3)).unwrap();

    buf.clear();
    assert_eq!(buf.head, 0);
    assert_eq!(buf.tail, 0);
    assert!(buf.buffer.iter().all(|x| x.is_none()));
}

#[test]
pub fn test_size(){
    let mut buf: CircularBuffer<i32> = CircularBuffer::new(10);
    buf.write(Some(1)).unwrap();
    buf.write(Some(2)).unwrap();
    buf.write(Some(3)).unwrap();

    assert_eq!(buf.size(), 3)
}

#[test]
pub fn test_overwrite(){
    let mut buf: CircularBuffer<i32> = CircularBuffer::new(3);
    buf.write(Some(1)).unwrap();
    buf.write(Some(2)).unwrap();
    buf.write(Some(3)).unwrap();

    buf.overwrite(Some(4));
    assert_eq!(buf.head, 1);
    assert_eq!(buf.tail, 1);
    assert_eq!(buf.buffer[0], Some(4));
}

#[test]
pub fn test_make_contiguous(){
    let mut buf: CircularBuffer<i32> = CircularBuffer::new(5);
    buf.write(Some(3)).unwrap();
    buf.write(None).unwrap();
    buf.write(None).unwrap();
    buf.write(Some(1)).unwrap();
    buf.write(Some(2)).unwrap();
    buf.head = 3;
    buf.tail = 1;

    let mut exp_buf: CircularBuffer<i32> = CircularBuffer::new(5);
    exp_buf.write(Some(1)).unwrap();
    exp_buf.write(Some(2)).unwrap();
    exp_buf.write(Some(3)).unwrap();

    buf.make_contiguous();
    assert_eq!(buf.buffer, exp_buf.buffer);
    assert_eq!(buf.tail, 4);
    assert_eq!(buf.head, 0);
}

#[test]
pub fn test_index_index_mut(){
    let mut buf: CircularBuffer<i32> = CircularBuffer::new(5);
    buf.write(Some(1)).unwrap();
    buf.write(Some(2)).unwrap();
    buf.write(Some(3)).unwrap();
    buf.write(Some(4)).unwrap();
    buf.write(Some(5)).unwrap();

    assert_eq!(buf[3], Some(4));
    buf[4] = Some(6);
    assert_eq!(buf[4], Some(6))
}