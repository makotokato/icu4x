import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XLocale } from "./ICU4XLocale";
import { ICU4XTransformResult } from "./ICU4XTransformResult";

/**

 * A locale expander.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid_transform/struct.LocaleExpander.html Rust documentation for `LocaleExpander`} for more information.
 */
export class ICU4XLocaleExpander {

  /**

   * Create a new {@link ICU4XLocaleExpander `ICU4XLocaleExpander`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid_transform/struct.LocaleExpander.html#method.try_new_unstable Rust documentation for `try_new_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create(provider: ICU4XDataProvider): ICU4XLocaleExpander | never;

  /**

   * FFI version of `LocaleExpander::maximize()`.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid_transform/struct.LocaleExpander.html#method.maximize Rust documentation for `maximize`} for more information.
   */
  maximize(locale: ICU4XLocale): ICU4XTransformResult;

  /**

   * FFI version of `LocaleExpander::minimize()`.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid_transform/struct.LocaleExpander.html#method.minimize Rust documentation for `minimize`} for more information.
   */
  minimize(locale: ICU4XLocale): ICU4XTransformResult;
}
