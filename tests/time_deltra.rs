use chrono::TimeDelta;

#[test]
fn add_asign() -> () {
    let mut time_delta = TimeDelta::seconds(5);
    time_delta += TimeDelta::seconds(10);
    assert_eq!(time_delta.num_seconds(), 15);
}

#[test]
fn sub_asign() -> () {
    let mut time_delta = TimeDelta::seconds(45);
    time_delta -= TimeDelta::seconds(2);
    assert_eq!(time_delta.num_seconds(), 43);
}
