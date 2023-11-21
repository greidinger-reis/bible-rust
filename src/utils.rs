pub trait SubscriptRepresentation {
    fn to_subscript(&self) -> String;
}

impl SubscriptRepresentation for usize {
    fn to_subscript(&self) -> String {
        self.to_string()
            .chars()
            .map(|c| match c {
                '0' => '₀',
                '1' => '₁',
                '2' => '₂',
                '3' => '₃',
                '4' => '₄',
                '5' => '₅',
                '6' => '₆',
                '7' => '₇',
                '8' => '₈',
                '9' => '₉',
                _ => c,
            })
            .collect()
    }
}
