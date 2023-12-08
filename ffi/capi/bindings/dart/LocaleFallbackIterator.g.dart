// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// An iterator over the locale under fallback.
///
/// See the [Rust documentation for `LocaleFallbackIterator`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbackIterator.html) for more information.
final class LocaleFallbackIterator implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  LocaleFallbackIterator._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XLocaleFallbackIterator_destroy'));

  /// Gets a snapshot of the current state of the locale.
  ///
  /// See the [Rust documentation for `get`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbackIterator.html#method.get) for more information.
  Locale get get {
    final result = _ICU4XLocaleFallbackIterator_get(_underlying);
    return Locale._(result);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocaleFallbackIterator_get =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>>('ICU4XLocaleFallbackIterator_get')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Performs one step of the fallback algorithm, mutating the locale.
  ///
  /// See the [Rust documentation for `step`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbackIterator.html#method.step) for more information.
  void step() {
    _ICU4XLocaleFallbackIterator_step(_underlying);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocaleFallbackIterator_step =
    _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>('ICU4XLocaleFallbackIterator_step')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}