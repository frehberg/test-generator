
#[macro_export]
macro_rules! utest {
    ( $id: ident, $setup:expr, $test:expr, $teardown:expr ) => {
       #[test]
       fn $id() {
            let context = std::panic::catch_unwind(|| {
                $setup()
            });

            assert!(context.is_ok());

            // unwrap the internal context item
            let ctx = match context {
                Ok(ctx) => ctx,
                Err(_) => unreachable!(),
            };

            let result = std::panic::catch_unwind(|| {
                $test(&ctx)
            });

            let finalizer = std::panic::catch_unwind(|| {
                $teardown(ctx)
            });

            assert!(result.is_ok());

            assert!(finalizer.is_ok());
       }
    };
}
