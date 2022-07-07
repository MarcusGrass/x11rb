use std::collections::HashMap;

use nix::libc::chdir;
use tiny_bench::black_box;

const KEY1: &[u8] = "_NET_WM_NUM_DESKTOPS".as_bytes();
const KEY2: &[u8] = "_ANOTHER_LONG_STRING".as_bytes();
const KEY3: &[u8] = "_MUZZA_ON_DA_BEAT".as_bytes();

fn main() {
    bench_smallmap_char_access();
    bench_hashmap_char_access();
    /*
    bench_hm_access();
    bench_smallmap_access();
     */
}

fn bench_smallmap_char_access() {
    let mut sm = smallmap::Map::new();
    for i in 0..14 {
        sm.insert(char::from(i), i);
    }
    tiny_bench::bench_labeled("sm_char_access", || {
        for i in 0..14 {
            assert_eq!(black_box(i), black_box(sm[black_box(&char::from(i))]));
        }
        for i in 14..28 {
            assert!(black_box(sm.get(black_box(&char::from(i))).is_none()))
        }
    });
}

fn bench_hashmap_char_access() {
    let mut sm = HashMap::new();
    for i in 0..14 {
        sm.insert(char::from(i), i);
    }
    tiny_bench::bench_labeled("hm_char_access", || {
        for i in 0..14 {
            assert_eq!(black_box(i), black_box(sm[black_box(&char::from(i))]));
        }
        for i in 14..28 {
            assert!(black_box(sm.get(black_box(&char::from(i))).is_none()))
        }
    });
}

fn bench_smallmap_access() {
    let mut sm = smallmap::Map::new();
    tiny_bench::bench_labeled("hm_access", || {
        sm.insert(black_box(KEY1), black_box(5));
        assert_eq!(black_box(sm.get(black_box(&KEY1))).unwrap(), &5);
        sm.remove(black_box(&KEY1));
        sm.insert(black_box(KEY2), black_box(5));
        assert_eq!(black_box(sm.get(black_box(&KEY2))).unwrap(), &5);
        sm.remove(black_box(&KEY2));
        sm.insert(black_box(KEY3), black_box(5));
        assert_eq!(black_box(sm.get(black_box(&KEY3))).unwrap(), &5);
        sm.remove(black_box(&KEY3));
    });
}

fn bench_hm_access() {
    let mut sm = HashMap::new();
    tiny_bench::bench_labeled("sm_access", || {
        sm.insert(black_box(KEY1), black_box(5));
        assert_eq!(black_box(sm.get(black_box(&KEY1))).unwrap(), &5);
        sm.remove(black_box(&KEY1));
        sm.insert(black_box(KEY2), black_box(5));
        assert_eq!(black_box(sm.get(black_box(&KEY2))).unwrap(), &5);
        sm.remove(black_box(&KEY2));
        sm.insert(black_box(KEY3), black_box(5));
        assert_eq!(black_box(sm.get(black_box(&KEY3))).unwrap(), &5);
        sm.remove(black_box(&KEY3));
    });
}
