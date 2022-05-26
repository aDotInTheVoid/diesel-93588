pub trait SelectNullableDsl {
        type Output;
        fn nullable(self) -> Self::Output;
}
