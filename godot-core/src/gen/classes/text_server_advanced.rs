#![doc = "Sidecar module for class [`TextServerAdvanced`][crate::engine::TextServerAdvanced].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextServerAdvanced` enums](https://docs.godotengine.org/en/stable/classes/class_textserveradvanced.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `TextServerAdvanced.`\n\nInherits [`TextServerExtension`][crate::engine::TextServerExtension].\n\nRelated symbols:\n\n* [`TextServerAdvancedVirtual`][crate::engine::TextServerAdvancedVirtual]: virtual methods\n\n\nSee also [Godot docs for `TextServerAdvanced`](https://docs.godotengine.org/en/stable/classes/class_textserveradvanced.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct TextServerAdvanced {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`TextServerAdvanced`][crate::engine::TextServerAdvanced].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TextServerAdvanced` methods](https://docs.godotengine.org/en/stable/classes/class_textserveradvanced.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait TextServerAdvancedVirtual:
        crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api
    {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder<Self>) {
            unimplemented!()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `#[base]` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base<Self::Base>) -> Self {
            unimplemented!()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GodotString {
            unimplemented!()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::engine::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: ObjectNotification) {
            unimplemented!()
        }
        fn has_feature(&self, feature: text_server::Feature) -> bool {
            unimplemented!()
        }
        fn get_name(&self) -> GodotString {
            unimplemented!()
        }
        fn get_features(&self) -> i64 {
            unimplemented!()
        }
        fn free_rid(&mut self, rid: Rid) {
            unimplemented!()
        }
        fn has(&mut self, rid: Rid) -> bool {
            unimplemented!()
        }
        fn load_support_data(&mut self, filename: GodotString) -> bool {
            unimplemented!()
        }
        fn get_support_data_filename(&self) -> GodotString {
            unimplemented!()
        }
        fn get_support_data_info(&self) -> GodotString {
            unimplemented!()
        }
        fn save_support_data(&self, filename: GodotString) -> bool {
            unimplemented!()
        }
        fn is_locale_right_to_left(&self, locale: GodotString) -> bool {
            unimplemented!()
        }
        fn name_to_tag(&self, name: GodotString) -> i64 {
            unimplemented!()
        }
        fn tag_to_name(&self, tag: i64) -> GodotString {
            unimplemented!()
        }
        fn create_font(&mut self) -> Rid {
            unimplemented!()
        }
        fn font_set_data(&mut self, font_rid: Rid, data: PackedByteArray) {
            unimplemented!()
        }
        fn font_set_face_index(&mut self, font_rid: Rid, face_index: i64) {
            unimplemented!()
        }
        fn font_get_face_index(&self, font_rid: Rid) -> i64 {
            unimplemented!()
        }
        fn font_get_face_count(&self, font_rid: Rid) -> i64 {
            unimplemented!()
        }
        fn font_set_style(&mut self, font_rid: Rid, style: text_server::FontStyle) {
            unimplemented!()
        }
        fn font_get_style(&self, font_rid: Rid) -> text_server::FontStyle {
            unimplemented!()
        }
        fn font_set_name(&mut self, font_rid: Rid, name: GodotString) {
            unimplemented!()
        }
        fn font_get_name(&self, font_rid: Rid) -> GodotString {
            unimplemented!()
        }
        fn font_set_style_name(&mut self, font_rid: Rid, name_style: GodotString) {
            unimplemented!()
        }
        fn font_get_style_name(&self, font_rid: Rid) -> GodotString {
            unimplemented!()
        }
        fn font_set_weight(&mut self, font_rid: Rid, weight: i64) {
            unimplemented!()
        }
        fn font_get_weight(&self, font_rid: Rid) -> i64 {
            unimplemented!()
        }
        fn font_set_stretch(&mut self, font_rid: Rid, stretch: i64) {
            unimplemented!()
        }
        fn font_get_stretch(&self, font_rid: Rid) -> i64 {
            unimplemented!()
        }
        fn font_set_antialiasing(
            &mut self,
            font_rid: Rid,
            antialiasing: text_server::FontAntialiasing,
        ) {
            unimplemented!()
        }
        fn font_get_antialiasing(&self, font_rid: Rid) -> text_server::FontAntialiasing {
            unimplemented!()
        }
        fn font_set_generate_mipmaps(&mut self, font_rid: Rid, generate_mipmaps: bool) {
            unimplemented!()
        }
        fn font_get_generate_mipmaps(&self, font_rid: Rid) -> bool {
            unimplemented!()
        }
        fn font_set_multichannel_signed_distance_field(&mut self, font_rid: Rid, msdf: bool) {
            unimplemented!()
        }
        fn font_is_multichannel_signed_distance_field(&self, font_rid: Rid) -> bool {
            unimplemented!()
        }
        fn font_set_msdf_pixel_range(&mut self, font_rid: Rid, msdf_pixel_range: i64) {
            unimplemented!()
        }
        fn font_get_msdf_pixel_range(&self, font_rid: Rid) -> i64 {
            unimplemented!()
        }
        fn font_set_msdf_size(&mut self, font_rid: Rid, msdf_size: i64) {
            unimplemented!()
        }
        fn font_get_msdf_size(&self, font_rid: Rid) -> i64 {
            unimplemented!()
        }
        fn font_set_fixed_size(&mut self, font_rid: Rid, fixed_size: i64) {
            unimplemented!()
        }
        fn font_get_fixed_size(&self, font_rid: Rid) -> i64 {
            unimplemented!()
        }
        fn font_set_allow_system_fallback(&mut self, font_rid: Rid, allow_system_fallback: bool) {
            unimplemented!()
        }
        fn font_is_allow_system_fallback(&self, font_rid: Rid) -> bool {
            unimplemented!()
        }
        fn font_set_force_autohinter(&mut self, font_rid: Rid, force_autohinter: bool) {
            unimplemented!()
        }
        fn font_is_force_autohinter(&self, font_rid: Rid) -> bool {
            unimplemented!()
        }
        fn font_set_hinting(&mut self, font_rid: Rid, hinting: text_server::Hinting) {
            unimplemented!()
        }
        fn font_get_hinting(&self, font_rid: Rid) -> text_server::Hinting {
            unimplemented!()
        }
        fn font_set_subpixel_positioning(
            &mut self,
            font_rid: Rid,
            subpixel_positioning: text_server::SubpixelPositioning,
        ) {
            unimplemented!()
        }
        fn font_get_subpixel_positioning(&self, font_rid: Rid) -> text_server::SubpixelPositioning {
            unimplemented!()
        }
        fn font_set_embolden(&mut self, font_rid: Rid, strength: f64) {
            unimplemented!()
        }
        fn font_get_embolden(&self, font_rid: Rid) -> f64 {
            unimplemented!()
        }
        fn font_set_transform(&mut self, font_rid: Rid, transform: Transform2D) {
            unimplemented!()
        }
        fn font_get_transform(&self, font_rid: Rid) -> Transform2D {
            unimplemented!()
        }
        fn font_set_variation_coordinates(
            &mut self,
            font_rid: Rid,
            variation_coordinates: Dictionary,
        ) {
            unimplemented!()
        }
        fn font_get_variation_coordinates(&self, font_rid: Rid) -> Dictionary {
            unimplemented!()
        }
        fn font_set_oversampling(&mut self, font_rid: Rid, oversampling: f64) {
            unimplemented!()
        }
        fn font_get_oversampling(&self, font_rid: Rid) -> f64 {
            unimplemented!()
        }
        fn font_get_size_cache_list(&self, font_rid: Rid) -> Array<Vector2i> {
            unimplemented!()
        }
        fn font_clear_size_cache(&mut self, font_rid: Rid) {
            unimplemented!()
        }
        fn font_remove_size_cache(&mut self, font_rid: Rid, size: Vector2i) {
            unimplemented!()
        }
        fn font_set_ascent(&mut self, font_rid: Rid, size: i64, ascent: f64) {
            unimplemented!()
        }
        fn font_get_ascent(&self, font_rid: Rid, size: i64) -> f64 {
            unimplemented!()
        }
        fn font_set_descent(&mut self, font_rid: Rid, size: i64, descent: f64) {
            unimplemented!()
        }
        fn font_get_descent(&self, font_rid: Rid, size: i64) -> f64 {
            unimplemented!()
        }
        fn font_set_underline_position(
            &mut self,
            font_rid: Rid,
            size: i64,
            underline_position: f64,
        ) {
            unimplemented!()
        }
        fn font_get_underline_position(&self, font_rid: Rid, size: i64) -> f64 {
            unimplemented!()
        }
        fn font_set_underline_thickness(
            &mut self,
            font_rid: Rid,
            size: i64,
            underline_thickness: f64,
        ) {
            unimplemented!()
        }
        fn font_get_underline_thickness(&self, font_rid: Rid, size: i64) -> f64 {
            unimplemented!()
        }
        fn font_set_scale(&mut self, font_rid: Rid, size: i64, scale: f64) {
            unimplemented!()
        }
        fn font_get_scale(&self, font_rid: Rid, size: i64) -> f64 {
            unimplemented!()
        }
        fn font_get_texture_count(&self, font_rid: Rid, size: Vector2i) -> i64 {
            unimplemented!()
        }
        fn font_clear_textures(&mut self, font_rid: Rid, size: Vector2i) {
            unimplemented!()
        }
        fn font_remove_texture(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64) {
            unimplemented!()
        }
        fn font_set_texture_image(
            &mut self,
            font_rid: Rid,
            size: Vector2i,
            texture_index: i64,
            image: Gd<Image>,
        ) {
            unimplemented!()
        }
        fn font_get_texture_image(
            &self,
            font_rid: Rid,
            size: Vector2i,
            texture_index: i64,
        ) -> Option<Gd<Image>> {
            unimplemented!()
        }
        fn font_set_texture_offsets(
            &mut self,
            font_rid: Rid,
            size: Vector2i,
            texture_index: i64,
            offset: PackedInt32Array,
        ) {
            unimplemented!()
        }
        fn font_get_texture_offsets(
            &self,
            font_rid: Rid,
            size: Vector2i,
            texture_index: i64,
        ) -> PackedInt32Array {
            unimplemented!()
        }
        fn font_get_glyph_list(&self, font_rid: Rid, size: Vector2i) -> PackedInt32Array {
            unimplemented!()
        }
        fn font_clear_glyphs(&mut self, font_rid: Rid, size: Vector2i) {
            unimplemented!()
        }
        fn font_remove_glyph(&mut self, font_rid: Rid, size: Vector2i, glyph: i64) {
            unimplemented!()
        }
        fn font_get_glyph_advance(&self, font_rid: Rid, size: i64, glyph: i64) -> Vector2 {
            unimplemented!()
        }
        fn font_set_glyph_advance(
            &mut self,
            font_rid: Rid,
            size: i64,
            glyph: i64,
            advance: Vector2,
        ) {
            unimplemented!()
        }
        fn font_get_glyph_offset(&self, font_rid: Rid, size: Vector2i, glyph: i64) -> Vector2 {
            unimplemented!()
        }
        fn font_set_glyph_offset(
            &mut self,
            font_rid: Rid,
            size: Vector2i,
            glyph: i64,
            offset: Vector2,
        ) {
            unimplemented!()
        }
        fn font_get_glyph_size(&self, font_rid: Rid, size: Vector2i, glyph: i64) -> Vector2 {
            unimplemented!()
        }
        fn font_set_glyph_size(
            &mut self,
            font_rid: Rid,
            size: Vector2i,
            glyph: i64,
            gl_size: Vector2,
        ) {
            unimplemented!()
        }
        fn font_get_glyph_uv_rect(&self, font_rid: Rid, size: Vector2i, glyph: i64) -> Rect2 {
            unimplemented!()
        }
        fn font_set_glyph_uv_rect(
            &mut self,
            font_rid: Rid,
            size: Vector2i,
            glyph: i64,
            uv_rect: Rect2,
        ) {
            unimplemented!()
        }
        fn font_get_glyph_texture_idx(&self, font_rid: Rid, size: Vector2i, glyph: i64) -> i64 {
            unimplemented!()
        }
        fn font_set_glyph_texture_idx(
            &mut self,
            font_rid: Rid,
            size: Vector2i,
            glyph: i64,
            texture_idx: i64,
        ) {
            unimplemented!()
        }
        fn font_get_glyph_texture_rid(&self, font_rid: Rid, size: Vector2i, glyph: i64) -> Rid {
            unimplemented!()
        }
        fn font_get_glyph_texture_size(
            &self,
            font_rid: Rid,
            size: Vector2i,
            glyph: i64,
        ) -> Vector2 {
            unimplemented!()
        }
        fn font_get_glyph_contours(&self, font_rid: Rid, size: i64, index: i64) -> Dictionary {
            unimplemented!()
        }
        fn font_get_kerning_list(&self, font_rid: Rid, size: i64) -> Array<Vector2i> {
            unimplemented!()
        }
        fn font_clear_kerning_map(&mut self, font_rid: Rid, size: i64) {
            unimplemented!()
        }
        fn font_remove_kerning(&mut self, font_rid: Rid, size: i64, glyph_pair: Vector2i) {
            unimplemented!()
        }
        fn font_set_kerning(
            &mut self,
            font_rid: Rid,
            size: i64,
            glyph_pair: Vector2i,
            kerning: Vector2,
        ) {
            unimplemented!()
        }
        fn font_get_kerning(&self, font_rid: Rid, size: i64, glyph_pair: Vector2i) -> Vector2 {
            unimplemented!()
        }
        fn font_get_glyph_index(
            &self,
            font_rid: Rid,
            size: i64,
            char: i64,
            variation_selector: i64,
        ) -> i64 {
            unimplemented!()
        }
        fn font_has_char(&self, font_rid: Rid, char: i64) -> bool {
            unimplemented!()
        }
        fn font_get_supported_chars(&self, font_rid: Rid) -> GodotString {
            unimplemented!()
        }
        fn font_render_range(&mut self, font_rid: Rid, size: Vector2i, start: i64, end: i64) {
            unimplemented!()
        }
        fn font_render_glyph(&mut self, font_rid: Rid, size: Vector2i, index: i64) {
            unimplemented!()
        }
        fn font_draw_glyph(
            &self,
            font_rid: Rid,
            canvas: Rid,
            size: i64,
            pos: Vector2,
            index: i64,
            color: Color,
        ) {
            unimplemented!()
        }
        fn font_draw_glyph_outline(
            &self,
            font_rid: Rid,
            canvas: Rid,
            size: i64,
            outline_size: i64,
            pos: Vector2,
            index: i64,
            color: Color,
        ) {
            unimplemented!()
        }
        fn font_is_language_supported(&self, font_rid: Rid, language: GodotString) -> bool {
            unimplemented!()
        }
        fn font_set_language_support_override(
            &mut self,
            font_rid: Rid,
            language: GodotString,
            supported: bool,
        ) {
            unimplemented!()
        }
        fn font_get_language_support_override(
            &mut self,
            font_rid: Rid,
            language: GodotString,
        ) -> bool {
            unimplemented!()
        }
        fn font_remove_language_support_override(&mut self, font_rid: Rid, language: GodotString) {
            unimplemented!()
        }
        fn font_get_language_support_overrides(&mut self, font_rid: Rid) -> PackedStringArray {
            unimplemented!()
        }
        fn font_is_script_supported(&self, font_rid: Rid, script: GodotString) -> bool {
            unimplemented!()
        }
        fn font_set_script_support_override(
            &mut self,
            font_rid: Rid,
            script: GodotString,
            supported: bool,
        ) {
            unimplemented!()
        }
        fn font_get_script_support_override(&mut self, font_rid: Rid, script: GodotString) -> bool {
            unimplemented!()
        }
        fn font_remove_script_support_override(&mut self, font_rid: Rid, script: GodotString) {
            unimplemented!()
        }
        fn font_get_script_support_overrides(&mut self, font_rid: Rid) -> PackedStringArray {
            unimplemented!()
        }
        fn font_set_opentype_feature_overrides(&mut self, font_rid: Rid, overrides: Dictionary) {
            unimplemented!()
        }
        fn font_get_opentype_feature_overrides(&self, font_rid: Rid) -> Dictionary {
            unimplemented!()
        }
        fn font_supported_feature_list(&self, font_rid: Rid) -> Dictionary {
            unimplemented!()
        }
        fn font_supported_variation_list(&self, font_rid: Rid) -> Dictionary {
            unimplemented!()
        }
        fn font_get_global_oversampling(&self) -> f64 {
            unimplemented!()
        }
        fn font_set_global_oversampling(&mut self, oversampling: f64) {
            unimplemented!()
        }
        fn get_hex_code_box_size(&self, size: i64, index: i64) -> Vector2 {
            unimplemented!()
        }
        fn draw_hex_code_box(
            &self,
            canvas: Rid,
            size: i64,
            pos: Vector2,
            index: i64,
            color: Color,
        ) {
            unimplemented!()
        }
        fn create_shaped_text(
            &mut self,
            direction: text_server::Direction,
            orientation: text_server::Orientation,
        ) -> Rid {
            unimplemented!()
        }
        fn shaped_text_clear(&mut self, shaped: Rid) {
            unimplemented!()
        }
        fn shaped_text_set_direction(&mut self, shaped: Rid, direction: text_server::Direction) {
            unimplemented!()
        }
        fn shaped_text_get_direction(&self, shaped: Rid) -> text_server::Direction {
            unimplemented!()
        }
        fn shaped_text_get_inferred_direction(&self, shaped: Rid) -> text_server::Direction {
            unimplemented!()
        }
        fn shaped_text_set_bidi_override(&mut self, shaped: Rid, override_: VariantArray) {
            unimplemented!()
        }
        fn shaped_text_set_custom_punctuation(&mut self, shaped: Rid, punct: GodotString) {
            unimplemented!()
        }
        fn shaped_text_get_custom_punctuation(&self, shaped: Rid) -> GodotString {
            unimplemented!()
        }
        fn shaped_text_set_orientation(
            &mut self,
            shaped: Rid,
            orientation: text_server::Orientation,
        ) {
            unimplemented!()
        }
        fn shaped_text_get_orientation(&self, shaped: Rid) -> text_server::Orientation {
            unimplemented!()
        }
        fn shaped_text_set_preserve_invalid(&mut self, shaped: Rid, enabled: bool) {
            unimplemented!()
        }
        fn shaped_text_get_preserve_invalid(&self, shaped: Rid) -> bool {
            unimplemented!()
        }
        fn shaped_text_set_preserve_control(&mut self, shaped: Rid, enabled: bool) {
            unimplemented!()
        }
        fn shaped_text_get_preserve_control(&self, shaped: Rid) -> bool {
            unimplemented!()
        }
        fn shaped_text_set_spacing(
            &mut self,
            shaped: Rid,
            spacing: text_server::SpacingType,
            value: i64,
        ) {
            unimplemented!()
        }
        fn shaped_text_get_spacing(&self, shaped: Rid, spacing: text_server::SpacingType) -> i64 {
            unimplemented!()
        }
        fn shaped_text_add_string(
            &mut self,
            shaped: Rid,
            text: GodotString,
            fonts: Array<Rid>,
            size: i64,
            opentype_features: Dictionary,
            language: GodotString,
            meta: Variant,
        ) -> bool {
            unimplemented!()
        }
        fn shaped_text_add_object(
            &mut self,
            shaped: Rid,
            key: Variant,
            size: Vector2,
            inline_align: global::InlineAlignment,
            length: i64,
            baseline: f64,
        ) -> bool {
            unimplemented!()
        }
        fn shaped_text_resize_object(
            &mut self,
            shaped: Rid,
            key: Variant,
            size: Vector2,
            inline_align: global::InlineAlignment,
            baseline: f64,
        ) -> bool {
            unimplemented!()
        }
        fn shaped_get_span_count(&self, shaped: Rid) -> i64 {
            unimplemented!()
        }
        fn shaped_get_span_meta(&self, shaped: Rid, index: i64) -> Variant {
            unimplemented!()
        }
        fn shaped_set_span_update_font(
            &mut self,
            shaped: Rid,
            index: i64,
            fonts: Array<Rid>,
            size: i64,
            opentype_features: Dictionary,
        ) {
            unimplemented!()
        }
        fn shaped_text_substr(&self, shaped: Rid, start: i64, length: i64) -> Rid {
            unimplemented!()
        }
        fn shaped_text_get_parent(&self, shaped: Rid) -> Rid {
            unimplemented!()
        }
        fn shaped_text_fit_to_width(
            &mut self,
            shaped: Rid,
            width: f64,
            jst_flags: text_server::JustificationFlag,
        ) -> f64 {
            unimplemented!()
        }
        fn shaped_text_tab_align(&mut self, shaped: Rid, tab_stops: PackedFloat32Array) -> f64 {
            unimplemented!()
        }
        fn shaped_text_shape(&mut self, shaped: Rid) -> bool {
            unimplemented!()
        }
        fn shaped_text_update_breaks(&mut self, shaped: Rid) -> bool {
            unimplemented!()
        }
        fn shaped_text_update_justification_ops(&mut self, shaped: Rid) -> bool {
            unimplemented!()
        }
        fn shaped_text_is_ready(&self, shaped: Rid) -> bool {
            unimplemented!()
        }
        fn shaped_text_get_glyph_count(&self, shaped: Rid) -> i64 {
            unimplemented!()
        }
        fn shaped_text_get_range(&self, shaped: Rid) -> Vector2i {
            unimplemented!()
        }
        fn shaped_text_get_line_breaks_adv(
            &self,
            shaped: Rid,
            width: PackedFloat32Array,
            start: i64,
            once: bool,
            break_flags: text_server::LineBreakFlag,
        ) -> PackedInt32Array {
            unimplemented!()
        }
        fn shaped_text_get_line_breaks(
            &self,
            shaped: Rid,
            width: f64,
            start: i64,
            break_flags: text_server::LineBreakFlag,
        ) -> PackedInt32Array {
            unimplemented!()
        }
        fn shaped_text_get_word_breaks(
            &self,
            shaped: Rid,
            grapheme_flags: text_server::GraphemeFlag,
        ) -> PackedInt32Array {
            unimplemented!()
        }
        fn shaped_text_get_trim_pos(&self, shaped: Rid) -> i64 {
            unimplemented!()
        }
        fn shaped_text_get_ellipsis_pos(&self, shaped: Rid) -> i64 {
            unimplemented!()
        }
        fn shaped_text_get_ellipsis_glyph_count(&self, shaped: Rid) -> i64 {
            unimplemented!()
        }
        fn shaped_text_overrun_trim_to_width(
            &mut self,
            shaped: Rid,
            width: f64,
            trim_flags: text_server::TextOverrunFlag,
        ) {
            unimplemented!()
        }
        fn shaped_text_get_objects(&self, shaped: Rid) -> VariantArray {
            unimplemented!()
        }
        fn shaped_text_get_object_rect(&self, shaped: Rid, key: Variant) -> Rect2 {
            unimplemented!()
        }
        fn shaped_text_get_size(&self, shaped: Rid) -> Vector2 {
            unimplemented!()
        }
        fn shaped_text_get_ascent(&self, shaped: Rid) -> f64 {
            unimplemented!()
        }
        fn shaped_text_get_descent(&self, shaped: Rid) -> f64 {
            unimplemented!()
        }
        fn shaped_text_get_width(&self, shaped: Rid) -> f64 {
            unimplemented!()
        }
        fn shaped_text_get_underline_position(&self, shaped: Rid) -> f64 {
            unimplemented!()
        }
        fn shaped_text_get_underline_thickness(&self, shaped: Rid) -> f64 {
            unimplemented!()
        }
        fn shaped_text_get_dominant_direction_in_range(
            &self,
            shaped: Rid,
            start: i64,
            end: i64,
        ) -> i64 {
            unimplemented!()
        }
        fn shaped_text_get_selection(
            &self,
            shaped: Rid,
            start: i64,
            end: i64,
        ) -> PackedVector2Array {
            unimplemented!()
        }
        fn shaped_text_hit_test_grapheme(&self, shaped: Rid, coord: f64) -> i64 {
            unimplemented!()
        }
        fn shaped_text_hit_test_position(&self, shaped: Rid, coord: f64) -> i64 {
            unimplemented!()
        }
        fn shaped_text_draw(
            &self,
            shaped: Rid,
            canvas: Rid,
            pos: Vector2,
            clip_l: f64,
            clip_r: f64,
            color: Color,
        ) {
            unimplemented!()
        }
        fn shaped_text_draw_outline(
            &self,
            shaped: Rid,
            canvas: Rid,
            pos: Vector2,
            clip_l: f64,
            clip_r: f64,
            outline_size: i64,
            color: Color,
        ) {
            unimplemented!()
        }
        fn shaped_text_get_grapheme_bounds(&self, shaped: Rid, pos: i64) -> Vector2 {
            unimplemented!()
        }
        fn shaped_text_next_grapheme_pos(&self, shaped: Rid, pos: i64) -> i64 {
            unimplemented!()
        }
        fn shaped_text_prev_grapheme_pos(&self, shaped: Rid, pos: i64) -> i64 {
            unimplemented!()
        }
        fn format_number(&self, string: GodotString, language: GodotString) -> GodotString {
            unimplemented!()
        }
        fn parse_number(&self, string: GodotString, language: GodotString) -> GodotString {
            unimplemented!()
        }
        fn percent_sign(&self, language: GodotString) -> GodotString {
            unimplemented!()
        }
        fn strip_diacritics(&self, string: GodotString) -> GodotString {
            unimplemented!()
        }
        fn is_valid_identifier(&self, string: GodotString) -> bool {
            unimplemented!()
        }
        fn string_get_word_breaks(
            &self,
            string: GodotString,
            language: GodotString,
            chars_per_line: i64,
        ) -> PackedInt32Array {
            unimplemented!()
        }
        fn is_confusable(&self, string: GodotString, dict: PackedStringArray) -> i64 {
            unimplemented!()
        }
        fn spoof_check(&self, string: GodotString) -> bool {
            unimplemented!()
        }
        fn string_to_upper(&self, string: GodotString, language: GodotString) -> GodotString {
            unimplemented!()
        }
        fn string_to_lower(&self, string: GodotString, language: GodotString) -> GodotString {
            unimplemented!()
        }
        fn parse_structured_text(
            &self,
            parser_type: text_server::StructuredTextParser,
            args: VariantArray,
            text: GodotString,
        ) -> Array<Vector3i> {
            unimplemented!()
        }
        fn cleanup(&mut self) {
            unimplemented!()
        }
    }
    impl TextServerAdvanced {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("TextServerAdvanced");
                let __object_ptr =
                    sys::interface_fn!(classdb_construct_object)(__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: ObjectNotification) {
            self.notification(i32::from(what) as i64, false);
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: ObjectNotification) {
            self.notification(i32::from(what) as i64, true);
        }
    }
    impl crate::obj::GodotClass for TextServerAdvanced {
        type Base = crate::engine::TextServerExtension;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "TextServerAdvanced";
    }
    impl crate::obj::EngineClass for TextServerAdvanced {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::TextServerExtension> for TextServerAdvanced {}
    impl crate::obj::Inherits<crate::engine::TextServer> for TextServerAdvanced {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for TextServerAdvanced {}
    impl crate::obj::Inherits<crate::engine::Object> for TextServerAdvanced {}
    impl std::ops::Deref for TextServerAdvanced {
        type Target = crate::engine::TextServerExtension;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for TextServerAdvanced {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_TextServerAdvanced {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::TextServerAdvanced> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::TextServerExtension> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::TextServer> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
