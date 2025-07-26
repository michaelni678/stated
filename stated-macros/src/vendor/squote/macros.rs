/// A macro like [`syn::parse_quote_spanned`], but you can interpolate
/// expressions with the `#{...}` syntax.
macro_rules! parse_squote {
    // ().
    (
        (output:
            $($output:tt)*
        )
        (input:
            [
                ( $($parenthesis:tt)* )
                $($rest:tt)*
            ]
            $($input:tt)*
        )
    ) => {
        parse_squote! {
            (output:
                [parenthesized: ]
                $($output)*
            )
            (input:
                [ $($parenthesis)* ]
                [ $($rest)* ]
                $($input)*
            )
        }
    };

    (
        (output:
            [parenthesized: $($parenthesis:tt)*]
            [ $($accumulated:tt)* ]
            $($output:tt)*
        )
        (input:
            []
            $($input:tt)*
        )
    ) => {
        parse_squote! {
            (output:
                [
                    $($accumulated)*
                    ( $($parenthesis)* )
                ]
                $($output)*
            )
            (input:
                $($input)*
            )
        }
    };

    // {}.
    (
        (output:
            $($output:tt)*
        )
        (input:
            [
                { $($brace:tt)* }
                $($rest:tt)*
            ]
            $($input:tt)*
        )
    ) => {
        parse_squote! {
            (output:
                [braced: ]
                $($output)*
            )
            (input:
                [ $($brace)* ]
                [ $($rest)* ]
                $($input)*
            )
        }
    };

    (
        (output:
            [braced: $($brace:tt)*]
            [ $($accumulated:tt)* ]
            $($output:tt)*
        )
        (input:
            []
            $($input:tt)*
        )
    ) => {
        parse_squote! {
            (output:
                [
                    $($accumulated)*
                    { $($brace)* }
                ]
                $($output)*
            )
            (input:
                $($input)*
            )
        }
    };

    // [].
    (
        (output:
            $($output:tt)*
        )
        (input:
            [
                [ $($bracket:tt)* ]
                $($rest:tt)*
            ]
            $($input:tt)*
        )
    ) => {
        parse_squote! {
            (output:
                [bracketed: ]
                $($output)*
            )
            (input:
                [ $($bracket)* ]
                [ $($rest)* ]
                $($input)*
            )
        }
    };

    (
        (output:
            [bracketed: $($bracket:tt)*]
            [ $($accumulated:tt)* ]
            $($output:tt)*
        )
        (input:
            []
            $($input:tt)*
        )
    ) => {
        parse_squote! {
            (output:
                [
                    $($accumulated)*
                    [ $($bracket)* ]
                ]
                $($output)*
            )
            (input:
                $($input)*
            )
        }
    };

    // #{...}
    (
        (output:
            [ $($accumulated:tt)* ]
            $($output:tt)*
        )
        (input:
            [
                #{ $expr:expr }
                $($rest:tt)*
            ]
            $($input:tt)*
        )
    ) => {
        match &$expr {
            interpolated => {
                parse_squote! {
                    (output:
                        [
                            $($accumulated)*
                            #interpolated
                        ]
                        $($output)*
                    )
                    (input:
                        [ $($rest)* ]
                        $($input)*
                    )
                }
            }
        }
    };

    // Other.
    (
        (output:
            [ $($accumulated:tt)* ]
            $($output:tt)*
        )
        (input:
            [
                $other:tt
                $($rest:tt)*
            ]
            $($input:tt)*
        )
    ) => {
        parse_squote! {
            (output:
                [
                    $($accumulated)*
                    $other
                ]
                $($output)*
            )
            (input:
                [ $($rest)* ]
                $($input)*
            )
        }
    };

    // End.
    (
        (output:
            [ $($output:tt)* ]
        )
        (input:
            []
        )
    ) => {
        ::syn::parse_quote_spanned! {
            $($output)*
        }
    };

    // Entry point with a span.
    (
        @$span:expr=>
        $($input:tt)*
    ) => {
        parse_squote! {
            (output:
                [$span=> ]
            )
            (input:
                [ $($input)* ]
            )
        }
    };

    // Entry point without a span.
    (
        $($input:tt)*
    ) => {
        parse_squote! {
            @::proc_macro2::Span::mixed_site()=>
            $($input)*
        }
    };
}

/// A macro like [`quote::quote_spanned`], but you can interpolate expressions
/// with the `#{...}` syntax.
macro_rules! squote {
    // ().
    (
        (output:
            $($output:tt)*
        )
        (input:
            [
                ( $($parenthesis:tt)* )
                $($rest:tt)*
            ]
            $($input:tt)*
        )
    ) => {
        squote! {
            (output:
                [parenthesized: ]
                $($output)*
            )
            (input:
                [ $($parenthesis)* ]
                [ $($rest)* ]
                $($input)*
            )
        }
    };

    (
        (output:
            [parenthesized: $($parenthesis:tt)*]
            [ $($accumulated:tt)* ]
            $($output:tt)*
        )
        (input:
            []
            $($input:tt)*
        )
    ) => {
        squote! {
            (output:
                [
                    $($accumulated)*
                    ( $($parenthesis)* )
                ]
                $($output)*
            )
            (input:
                $($input)*
            )
        }
    };

    // {}.
    (
        (output:
            $($output:tt)*
        )
        (input:
            [
                { $($brace:tt)* }
                $($rest:tt)*
            ]
            $($input:tt)*
        )
    ) => {
        squote! {
            (output:
                [braced: ]
                $($output)*
            )
            (input:
                [ $($brace)* ]
                [ $($rest)* ]
                $($input)*
            )
        }
    };

    (
        (output:
            [braced: $($brace:tt)*]
            [ $($accumulated:tt)* ]
            $($output:tt)*
        )
        (input:
            []
            $($input:tt)*
        )
    ) => {
        squote! {
            (output:
                [
                    $($accumulated)*
                    { $($brace)* }
                ]
                $($output)*
            )
            (input:
                $($input)*
            )
        }
    };

    // [].
    (
        (output:
            $($output:tt)*
        )
        (input:
            [
                [ $($bracket:tt)* ]
                $($rest:tt)*
            ]
            $($input:tt)*
        )
    ) => {
        squote! {
            (output:
                [bracketed: ]
                $($output)*
            )
            (input:
                [ $($bracket)* ]
                [ $($rest)* ]
                $($input)*
            )
        }
    };

    (
        (output:
            [bracketed: $($bracket:tt)*]
            [ $($accumulated:tt)* ]
            $($output:tt)*
        )
        (input:
            []
            $($input:tt)*
        )
    ) => {
        squote! {
            (output:
                [
                    $($accumulated)*
                    [ $($bracket)* ]
                ]
                $($output)*
            )
            (input:
                $($input)*
            )
        }
    };

    // #{...}.
    (
        (output:
            [ $($accumulated:tt)* ]
            $($output:tt)*
        )
        (input:
            [
                #{ $expr:expr }
                $($rest:tt)*
            ]
            $($input:tt)*
        )
    ) => {
        match &$expr {
            interpolated => {
                squote! {
                    (output:
                        [
                            $($accumulated)*
                            #interpolated
                        ]
                        $($output)*
                    )
                    (input:
                        [ $($rest)* ]
                        $($input)*
                    )
                }
            }
        }
    };

    // Other.
    (
        (output:
            [ $($accumulated:tt)* ]
            $($output:tt)*
        )
        (input:
            [
                $other:tt
                $($rest:tt)*
            ]
            $($input:tt)*
        )
    ) => {
        squote! {
            (output:
                [
                    $($accumulated)*
                    $other
                ]
                $($output)*
            )
            (input:
                [ $($rest)* ]
                $($input)*
            )
        }
    };

    // End.
    (
        (output:
            [ $($output:tt)* ]
        )
        (input:
            []
        )
    ) => {
        ::quote::quote_spanned! {
            $($output)*
        }
    };

    // Entry point with a span.
    (
        @$span:expr=>
        $($input:tt)*
    ) => {
        squote! {
            (output:
                [$span=> ]
            )
            (input:
                [ $($input)* ]
            )
        }
    };

    // Entry point without a span.
    (
        $($input:tt)*
    ) => {
        squote! {
            @::proc_macro2::Span::mixed_site()=>
            $($input)*
        }
    };
}

pub(crate) use {parse_squote, squote};
