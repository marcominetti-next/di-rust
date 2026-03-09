use rudi::{modules, Context, ProviderEntry, Scope};

#[test]
fn standalone_variables() {
    macro_rules! single_test {
        ($method:ident, $variant:ident) => {
            let cx = Context::options()
                .$method(42i32)
                .$method(true)
                .$method("Hello world")
                .create(modules![]);

            assert_eq!(cx.single_registry_len(), 3);

            assert_eq!(cx.get_single::<i32>(), &42);
            assert!(*cx.get_single::<bool>());
            assert_eq!(cx.get_single::<&str>(), &"Hello world");

            assert_eq!(cx.single_registry_len(), 3);

            cx.registry_entries().iter().for_each(|(_, entry)| {
                let provider = match entry {
                    ProviderEntry::Provider(p) | ProviderEntry::WithInstance(p, _) => p,
                };
                assert!(provider.definition().scope == Scope::$variant);
                assert!(!provider.eager_create());
            });
        };
    }

    {
        single_test!(singleton, Singleton);
    }

    {
        single_test!(single_owner, SingleOwner);
    }
}
