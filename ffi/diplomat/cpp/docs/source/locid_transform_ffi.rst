``locid_transform::ffi``
========================

.. cpp:class:: ICU4XLocaleCanonicalizer

    A locale canonicalizer.

    See the `Rust documentation for LocaleCanonicalizer <https://unicode-org.github.io/icu4x-docs/doc/icu/locid_transform/struct.LocaleCanonicalizer.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XLocaleCanonicalizer, ICU4XError> create(const ICU4XDataProvider& provider)

        Create a new :cpp:class:`ICU4XLocaleCanonicalizer`.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/locid_transform/struct.LocaleCanonicalizer.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: ICU4XTransformResult canonicalize(ICU4XLocale& locale) const

        FFI version of ``LocaleCanonicalizer::canonicalize()``.

        See the `Rust documentation for canonicalize <https://unicode-org.github.io/icu4x-docs/doc/icu/locid_transform/struct.LocaleCanonicalizer.html#method.canonicalize>`__ for more information.


.. cpp:class:: ICU4XLocaleExpander

    A locale expander.

    See the `Rust documentation for LocaleExpander <https://unicode-org.github.io/icu4x-docs/doc/icu/locid_transform/struct.LocaleExpander.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XLocaleExpander, ICU4XError> create(const ICU4XDataProvider& provider)

        Create a new :cpp:class:`ICU4XLocaleExpander`.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/locid_transform/struct.LocaleExpander.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: ICU4XTransformResult maximize(ICU4XLocale& locale) const

        FFI version of ``LocaleExpander::maximize()``.

        See the `Rust documentation for maximize <https://unicode-org.github.io/icu4x-docs/doc/icu/locid_transform/struct.LocaleExpander.html#method.maximize>`__ for more information.


    .. cpp:function:: ICU4XTransformResult minimize(ICU4XLocale& locale) const

        FFI version of ``LocaleExpander::minimize()``.

        See the `Rust documentation for minimize <https://unicode-org.github.io/icu4x-docs/doc/icu/locid_transform/struct.LocaleExpander.html#method.minimize>`__ for more information.


.. cpp:enum-struct:: ICU4XTransformResult

    FFI version of ``TransformResult``.

    See the `Rust documentation for TransformResult <https://unicode-org.github.io/icu4x-docs/doc/icu/locid_transform/enum.TransformResult.html>`__ for more information.


    .. cpp:enumerator:: Modified

    .. cpp:enumerator:: Unmodified
