use gatenative::cpu_build_exec::*;
use gatenative::*;

#[test]
fn test_parent_data_holder() {
    let mut pdh = ParentDataHolder::new(
        5..22,
        CPUDataHolder::new((0..30).map(|i| 40 + 5 * i).collect()),
    );
    assert_eq!(17, pdh.len());
    {
        let r = pdh.get();
        assert_eq!((5..22).map(|i| 40 + 5 * i).collect::<Vec<_>>(), r.get());
    }
    pdh.set_range(6..12);
    assert_eq!(6, pdh.len());
    {
        let r = pdh.get();
        assert_eq!((11..17).map(|i| 40 + 5 * i).collect::<Vec<_>>(), r.get());
    }
    pdh.set_range(3..15);
    assert_eq!(12, pdh.len());
    {
        let r = pdh.get();
        assert_eq!((8..20).map(|i| 40 + 5 * i).collect::<Vec<_>>(), r.get());
    }
    let pdh2 = pdh.copy();
    assert_eq!(12, pdh2.len());
    {
        let r = pdh2.get();
        assert_eq!((8..20).map(|i| 40 + 5 * i).collect::<Vec<_>>(), r.get());
    }
}
