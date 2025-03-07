var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
import initModule from '../pkg/syntastica-js.js';
const PTR_SIZE = Float32Array.BYTES_PER_ELEMENT;
let Module = null;
/**
 * Load the requested languages.
 *
 * This function _must_ be called before any of the others. It accepts a list of languages to load. The function can
 * be called multiple times to re-initialize with a different set of languages, but this is generally not recommended.
 *
 * @param languages - An optional list of languages to load. By default, all languages will be loaded.
 * See [here](https://rubixdev.github.io/syntastica/syntastica_parsers_git/) for a list of supported languages.
 */
export function init(languages) {
    return __awaiter(this, void 0, void 0, function* () {
        if (Module === null) {
            Module = yield initModule();
        }
        if (languages === undefined) {
            Module._init(0, 0);
        }
        else {
            // allocate an array
            const list_len = languages.length;
            const list_ptr = Module._malloc(list_len * PTR_SIZE);
            // store pointers to the string in the array
            for (let i = 0; i < list_len; i++) {
                Module.setValue(list_ptr + i * PTR_SIZE, Module.stringToNewUTF8(languages[i]), '*');
            }
            // call `init`
            Module._init(list_ptr, list_len);
            // free everything
            for (let i = 0; i < list_len; i++) {
                Module._free(Module.getValue(list_ptr + i * PTR_SIZE, '*'));
            }
            Module._free(list_ptr);
        }
    });
}
/**
 * Highlight code and render to the requested format.
 *
 * If you plan to highlight the same input multiple times, use {@link process} and {@link render} instead.
 *
 * @param code - The code to highlight.
 *
 * @param language - The name of the code's language.
 *
 * The language must have been loaded previously by calling {@link init}.
 *
 * @param theme - The name of the theme to use.
 *
 * All themes from {@link https://rubixdev.github.io/syntastica/syntastica_themes/ | the default collection}
 * are supported. The theme name is equivalent to its Rust path specifier, so for example the gruvbox dark theme
 * is named `gruvbox::dark`.
 *
 * @param renderer - The renderer to use.
 *
 * The renderer name is either `HTML` or `Terminal` in any casing. To specify a background color
 * for the terminal renderer, append a hex color literal like `terminal#282828` or `Terminal#fff`.
 *
 * By default, the `HTML` renderer will be used.
 *
 * @returns The highlighted code as HTML code.
 *
 * See {@link https://rubixdev.github.io/syntastica-ci-test/syntastica/renderer/struct.HtmlRenderer.html | here} for
 * more information on the output.
 */
export function highlight(code, language, theme, renderer = 'HTML') {
    const code_ptr = Module.stringToNewUTF8(code);
    const language_ptr = Module.stringToNewUTF8(language);
    const theme_ptr = Module.stringToNewUTF8(theme);
    const renderer_ptr = Module.stringToNewUTF8(renderer);
    const result_ptr = Module._highlight(code_ptr, language_ptr, theme_ptr, renderer_ptr);
    const result = Module.UTF8ToString(result_ptr);
    Module._free(code_ptr);
    Module._free(language_ptr);
    Module._free(theme_ptr);
    Module._free(renderer_ptr);
    Module._free(result_ptr);
    return result;
}
/**
 * Prepare code for rendering multiple times.
 *
 * @param code - The code to highlight.
 *
 * @param language - The name of the code's language.
 *
 * The language must have been loaded previously by calling {@link init}.
 */
export function process(code, language) {
    const code_ptr = Module.stringToNewUTF8(code);
    const language_ptr = Module.stringToNewUTF8(language);
    Module._process(code_ptr, language_ptr);
    Module._free(code_ptr);
    Module._free(language_ptr);
}
/**
 * Render code that was previously processed by calling {@link process}.
 *
 * @param theme - The name of the theme to use.
 *
 * All themes from {@link https://rubixdev.github.io/syntastica/syntastica_themes/ | the default collection}
 * are supported. The theme name is equivalent to its Rust path specifier, so for example the gruvbox dark theme
 * is named `gruvbox::dark`.
 *
 * @param renderer - The renderer to use.
 *
 * The renderer name is either `HTML` or `Terminal` in any casing. To specify a background color
 * for the terminal renderer, append a hex color literal like `terminal#282828` or `Terminal#fff`.
 *
 * By default, the `HTML` renderer will be used.
 *
 * @returns The highlighted code in the requested format.
 */
export function render(theme, renderer = 'HTML') {
    const theme_ptr = Module.stringToNewUTF8(theme);
    const renderer_ptr = Module.stringToNewUTF8(renderer);
    const result_ptr = Module._render(theme_ptr, renderer_ptr);
    const result = Module.UTF8ToString(result_ptr);
    Module._free(theme_ptr);
    Module._free(renderer_ptr);
    Module._free(result_ptr);
    return result;
}
export default { init, highlight, process, render };
// DISCLAIMER: All code below this line is generated with `cargo xtask codegen js-list`
// in the syntastica workspace. Do not edit this code manually!
/**
 * A list of all supported languages.
 *
 * @see The {@link Language} type.
 */
export const LANGUAGES = [
    'asm',
    'bash',
    'c',
    'cpp',
    'css',
    'go',
    'html',
    'java',
    'javascript',
    'json',
    'python',
    'regex',
    'rust',
    'tsx',
    'typescript',
];
/**
 * A list of all valid themes.
 *
 * @see The {@link Theme} type.
 */
export const THEMES = [
    'gruvbox::dark',
    'gruvbox::light',
    'one::cool',
    'one::dark',
    'one::darker',
    'one::deep',
    'one::light',
    'one::warm',
    'one::warmer',
];
