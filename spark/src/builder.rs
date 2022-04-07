#![allow(clippy::wrong_self_convention)]

use super::vk;
use std::{
    ffi::CStr,
    marker::PhantomData,
    mem,
    ops::Deref,
    os::raw::{c_char, c_int, c_void},
    ptr,
};

pub trait Builder<'a> {
    type Type;
    fn builder() -> Self::Type;
}

unsafe fn insert_next(mut head: *mut vk::BaseOutStructure, other: *mut vk::BaseOutStructure) {
    assert!((*other).p_next.is_null());
    (*other).p_next = (*head).p_next;
    (*head).p_next = other as *mut _;
}

impl Builder<'_> for vk::BaseOutStructure {
    type Type = BaseOutStructureBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BaseOutStructureBuilder {
    inner: vk::BaseOutStructure,
}
impl BaseOutStructureBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut vk::BaseOutStructure) -> Self {
        self.inner.p_next = p_next;
        self
    }
}
impl Deref for BaseOutStructureBuilder {
    type Target = vk::BaseOutStructure;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BaseInStructure {
    type Type = BaseInStructureBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BaseInStructureBuilder<'a> {
    inner: vk::BaseInStructure,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BaseInStructureBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: Option<&'a vk::BaseInStructure>) -> Self {
        self.inner.p_next = p_next.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for BaseInStructureBuilder<'a> {
    type Target = vk::BaseInStructure;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ApplicationInfo {
    type Type = ApplicationInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ApplicationInfoBuilder<'a> {
    inner: vk::ApplicationInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ApplicationInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_application_name(mut self, p_application_name: Option<&'a CStr>) -> Self {
        self.inner.p_application_name = p_application_name.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn application_version(mut self, application_version: u32) -> Self {
        self.inner.application_version = application_version;
        self
    }
    pub fn p_engine_name(mut self, p_engine_name: Option<&'a CStr>) -> Self {
        self.inner.p_engine_name = p_engine_name.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn engine_version(mut self, engine_version: u32) -> Self {
        self.inner.engine_version = engine_version;
        self
    }
    pub fn api_version(mut self, api_version: vk::Version) -> Self {
        self.inner.api_version = api_version;
        self
    }
}
impl<'a> Deref for ApplicationInfoBuilder<'a> {
    type Target = vk::ApplicationInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AllocationCallbacks {
    type Type = AllocationCallbacksBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AllocationCallbacksBuilder {
    inner: vk::AllocationCallbacks,
}
impl AllocationCallbacksBuilder {
    pub fn p_user_data(mut self, p_user_data: *mut c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
    pub fn pfn_allocation(mut self, pfn_allocation: vk::FnAllocationFunction) -> Self {
        self.inner.pfn_allocation = Some(pfn_allocation);
        self
    }
    pub fn pfn_reallocation(mut self, pfn_reallocation: vk::FnReallocationFunction) -> Self {
        self.inner.pfn_reallocation = Some(pfn_reallocation);
        self
    }
    pub fn pfn_free(mut self, pfn_free: vk::FnFreeFunction) -> Self {
        self.inner.pfn_free = Some(pfn_free);
        self
    }
    pub fn pfn_internal_allocation(
        mut self,
        pfn_internal_allocation: Option<vk::FnInternalAllocationNotification>,
    ) -> Self {
        self.inner.pfn_internal_allocation = pfn_internal_allocation;
        self
    }
    pub fn pfn_internal_free(mut self, pfn_internal_free: Option<vk::FnInternalFreeNotification>) -> Self {
        self.inner.pfn_internal_free = pfn_internal_free;
        self
    }
}
impl Deref for AllocationCallbacksBuilder {
    type Target = vk::AllocationCallbacks;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceQueueCreateInfo {
    type Type = DeviceQueueCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DeviceQueueCreateInfoNext {}
#[derive(Default)]
pub struct DeviceQueueCreateInfoBuilder<'a> {
    inner: vk::DeviceQueueCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DeviceQueueCreateInfoBuilder<'a> {
    pub fn insert_next<T: DeviceQueueCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DeviceQueueCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.inner.queue_family_index = queue_family_index;
        self
    }
    pub fn p_queue_priorities(mut self, p_queue_priorities: &'a [f32]) -> Self {
        self.inner.queue_count = p_queue_priorities.len() as u32;
        self.inner.p_queue_priorities = p_queue_priorities.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for DeviceQueueCreateInfoBuilder<'a> {
    type Target = vk::DeviceQueueCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceCreateInfo {
    type Type = DeviceCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DeviceCreateInfoNext {}
#[derive(Default)]
pub struct DeviceCreateInfoBuilder<'a> {
    inner: vk::DeviceCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DeviceCreateInfoBuilder<'a> {
    pub fn insert_next<T: DeviceCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DeviceCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_queue_create_infos(mut self, p_queue_create_infos: &'a [vk::DeviceQueueCreateInfo]) -> Self {
        self.inner.queue_create_info_count = p_queue_create_infos.len() as u32;
        self.inner.p_queue_create_infos = p_queue_create_infos.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn pp_enabled_layer_names(mut self, pp_enabled_layer_names: &'a [*const c_char]) -> Self {
        self.inner.enabled_layer_count = pp_enabled_layer_names.len() as u32;
        self.inner.pp_enabled_layer_names = pp_enabled_layer_names.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn pp_enabled_extension_names(mut self, pp_enabled_extension_names: &'a [*const c_char]) -> Self {
        self.inner.enabled_extension_count = pp_enabled_extension_names.len() as u32;
        self.inner.pp_enabled_extension_names = pp_enabled_extension_names
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_enabled_features(mut self, p_enabled_features: Option<&'a vk::PhysicalDeviceFeatures>) -> Self {
        self.inner.p_enabled_features = p_enabled_features.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for DeviceCreateInfoBuilder<'a> {
    type Target = vk::DeviceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::InstanceCreateInfo {
    type Type = InstanceCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait InstanceCreateInfoNext {}
#[derive(Default)]
pub struct InstanceCreateInfoBuilder<'a> {
    inner: vk::InstanceCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> InstanceCreateInfoBuilder<'a> {
    pub fn insert_next<T: InstanceCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::InstanceCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_application_info(mut self, p_application_info: Option<&'a vk::ApplicationInfo>) -> Self {
        self.inner.p_application_info = p_application_info.map_or(ptr::null(), |p| p);
        self
    }
    pub fn pp_enabled_layer_names(mut self, pp_enabled_layer_names: &'a [*const c_char]) -> Self {
        self.inner.enabled_layer_count = pp_enabled_layer_names.len() as u32;
        self.inner.pp_enabled_layer_names = pp_enabled_layer_names.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn pp_enabled_extension_names(mut self, pp_enabled_extension_names: &'a [*const c_char]) -> Self {
        self.inner.enabled_extension_count = pp_enabled_extension_names.len() as u32;
        self.inner.pp_enabled_extension_names = pp_enabled_extension_names
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for InstanceCreateInfoBuilder<'a> {
    type Target = vk::InstanceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::MemoryAllocateInfo {
    type Type = MemoryAllocateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait MemoryAllocateInfoNext {}
#[derive(Default)]
pub struct MemoryAllocateInfoBuilder<'a> {
    inner: vk::MemoryAllocateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> MemoryAllocateInfoBuilder<'a> {
    pub fn insert_next<T: MemoryAllocateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn allocation_size(mut self, allocation_size: vk::DeviceSize) -> Self {
        self.inner.allocation_size = allocation_size;
        self
    }
    pub fn memory_type_index(mut self, memory_type_index: u32) -> Self {
        self.inner.memory_type_index = memory_type_index;
        self
    }
}
impl<'a> Deref for MemoryAllocateInfoBuilder<'a> {
    type Target = vk::MemoryAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::MappedMemoryRange {
    type Type = MappedMemoryRangeBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MappedMemoryRangeBuilder {
    inner: vk::MappedMemoryRange,
}
impl MappedMemoryRangeBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
}
impl Deref for MappedMemoryRangeBuilder {
    type Target = vk::MappedMemoryRange;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::WriteDescriptorSet {
    type Type = WriteDescriptorSetBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait WriteDescriptorSetNext {}
#[derive(Default)]
pub struct WriteDescriptorSetBuilder<'a> {
    inner: vk::WriteDescriptorSet,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> WriteDescriptorSetBuilder<'a> {
    pub fn insert_next<T: WriteDescriptorSetNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dst_set(mut self, dst_set: vk::DescriptorSet) -> Self {
        self.inner.dst_set = Some(dst_set);
        self
    }
    pub fn dst_binding(mut self, dst_binding: u32) -> Self {
        self.inner.dst_binding = dst_binding;
        self
    }
    pub fn dst_array_element(mut self, dst_array_element: u32) -> Self {
        self.inner.dst_array_element = dst_array_element;
        self
    }
    pub fn p_image_info(mut self, p_image_info: &'a [vk::DescriptorImageInfo]) -> Self {
        self.inner.descriptor_count = p_image_info.len() as u32;
        self.inner.p_image_info = p_image_info.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_buffer_info(mut self, p_buffer_info: &'a [vk::DescriptorBufferInfo]) -> Self {
        self.inner.descriptor_count = p_buffer_info.len() as u32;
        self.inner.p_buffer_info = p_buffer_info.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_texel_buffer_view(mut self, p_texel_buffer_view: &'a [vk::BufferView]) -> Self {
        self.inner.descriptor_count = p_texel_buffer_view.len() as u32;
        self.inner.p_texel_buffer_view = p_texel_buffer_view.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn descriptor_type(mut self, descriptor_type: vk::DescriptorType) -> Self {
        self.inner.descriptor_type = descriptor_type;
        self
    }
}
impl<'a> Deref for WriteDescriptorSetBuilder<'a> {
    type Target = vk::WriteDescriptorSet;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CopyDescriptorSet {
    type Type = CopyDescriptorSetBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyDescriptorSetBuilder {
    inner: vk::CopyDescriptorSet,
}
impl CopyDescriptorSetBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_set(mut self, src_set: vk::DescriptorSet) -> Self {
        self.inner.src_set = Some(src_set);
        self
    }
    pub fn src_binding(mut self, src_binding: u32) -> Self {
        self.inner.src_binding = src_binding;
        self
    }
    pub fn src_array_element(mut self, src_array_element: u32) -> Self {
        self.inner.src_array_element = src_array_element;
        self
    }
    pub fn dst_set(mut self, dst_set: vk::DescriptorSet) -> Self {
        self.inner.dst_set = Some(dst_set);
        self
    }
    pub fn dst_binding(mut self, dst_binding: u32) -> Self {
        self.inner.dst_binding = dst_binding;
        self
    }
    pub fn dst_array_element(mut self, dst_array_element: u32) -> Self {
        self.inner.dst_array_element = dst_array_element;
        self
    }
    pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.inner.descriptor_count = descriptor_count;
        self
    }
}
impl Deref for CopyDescriptorSetBuilder {
    type Target = vk::CopyDescriptorSet;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BufferCreateInfo {
    type Type = BufferCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait BufferCreateInfoNext {}
#[derive(Default)]
pub struct BufferCreateInfoBuilder<'a> {
    inner: vk::BufferCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BufferCreateInfoBuilder<'a> {
    pub fn insert_next<T: BufferCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::BufferCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
    pub fn usage(mut self, usage: vk::BufferUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn sharing_mode(mut self, sharing_mode: vk::SharingMode) -> Self {
        self.inner.sharing_mode = sharing_mode;
        self
    }
    pub fn p_queue_family_indices(mut self, p_queue_family_indices: &'a [u32]) -> Self {
        self.inner.queue_family_index_count = p_queue_family_indices.len() as u32;
        self.inner.p_queue_family_indices = p_queue_family_indices.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for BufferCreateInfoBuilder<'a> {
    type Target = vk::BufferCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BufferViewCreateInfo {
    type Type = BufferViewCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferViewCreateInfoBuilder {
    inner: vk::BufferViewCreateInfo,
}
impl BufferViewCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::BufferViewCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn range(mut self, range: vk::DeviceSize) -> Self {
        self.inner.range = range;
        self
    }
}
impl Deref for BufferViewCreateInfoBuilder {
    type Target = vk::BufferViewCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::MemoryBarrier {
    type Type = MemoryBarrierBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryBarrierBuilder {
    inner: vk::MemoryBarrier,
}
impl MemoryBarrierBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: vk::AccessFlags) -> Self {
        self.inner.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: vk::AccessFlags) -> Self {
        self.inner.dst_access_mask = dst_access_mask;
        self
    }
}
impl Deref for MemoryBarrierBuilder {
    type Target = vk::MemoryBarrier;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BufferMemoryBarrier {
    type Type = BufferMemoryBarrierBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferMemoryBarrierBuilder {
    inner: vk::BufferMemoryBarrier,
}
impl BufferMemoryBarrierBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: vk::AccessFlags) -> Self {
        self.inner.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: vk::AccessFlags) -> Self {
        self.inner.dst_access_mask = dst_access_mask;
        self
    }
    pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.inner.src_queue_family_index = src_queue_family_index;
        self
    }
    pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.inner.dst_queue_family_index = dst_queue_family_index;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
}
impl Deref for BufferMemoryBarrierBuilder {
    type Target = vk::BufferMemoryBarrier;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageMemoryBarrier {
    type Type = ImageMemoryBarrierBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ImageMemoryBarrierNext {}
#[derive(Default)]
pub struct ImageMemoryBarrierBuilder<'a> {
    inner: vk::ImageMemoryBarrier,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ImageMemoryBarrierBuilder<'a> {
    pub fn insert_next<T: ImageMemoryBarrierNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: vk::AccessFlags) -> Self {
        self.inner.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: vk::AccessFlags) -> Self {
        self.inner.dst_access_mask = dst_access_mask;
        self
    }
    pub fn old_layout(mut self, old_layout: vk::ImageLayout) -> Self {
        self.inner.old_layout = old_layout;
        self
    }
    pub fn new_layout(mut self, new_layout: vk::ImageLayout) -> Self {
        self.inner.new_layout = new_layout;
        self
    }
    pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.inner.src_queue_family_index = src_queue_family_index;
        self
    }
    pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.inner.dst_queue_family_index = dst_queue_family_index;
        self
    }
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
    pub fn subresource_range(mut self, subresource_range: vk::ImageSubresourceRange) -> Self {
        self.inner.subresource_range = subresource_range;
        self
    }
}
impl<'a> Deref for ImageMemoryBarrierBuilder<'a> {
    type Target = vk::ImageMemoryBarrier;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageCreateInfo {
    type Type = ImageCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ImageCreateInfoNext {}
#[derive(Default)]
pub struct ImageCreateInfoBuilder<'a> {
    inner: vk::ImageCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ImageCreateInfoBuilder<'a> {
    pub fn insert_next<T: ImageCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ImageCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn image_type(mut self, image_type: vk::ImageType) -> Self {
        self.inner.image_type = image_type;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn extent(mut self, extent: vk::Extent3D) -> Self {
        self.inner.extent = extent;
        self
    }
    pub fn mip_levels(mut self, mip_levels: u32) -> Self {
        self.inner.mip_levels = mip_levels;
        self
    }
    pub fn array_layers(mut self, array_layers: u32) -> Self {
        self.inner.array_layers = array_layers;
        self
    }
    pub fn samples(mut self, samples: vk::SampleCountFlags) -> Self {
        self.inner.samples = samples;
        self
    }
    pub fn tiling(mut self, tiling: vk::ImageTiling) -> Self {
        self.inner.tiling = tiling;
        self
    }
    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn sharing_mode(mut self, sharing_mode: vk::SharingMode) -> Self {
        self.inner.sharing_mode = sharing_mode;
        self
    }
    pub fn p_queue_family_indices(mut self, p_queue_family_indices: &'a [u32]) -> Self {
        self.inner.queue_family_index_count = p_queue_family_indices.len() as u32;
        self.inner.p_queue_family_indices = p_queue_family_indices.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn initial_layout(mut self, initial_layout: vk::ImageLayout) -> Self {
        self.inner.initial_layout = initial_layout;
        self
    }
}
impl<'a> Deref for ImageCreateInfoBuilder<'a> {
    type Target = vk::ImageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageViewCreateInfo {
    type Type = ImageViewCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ImageViewCreateInfoNext {}
#[derive(Default)]
pub struct ImageViewCreateInfoBuilder<'a> {
    inner: vk::ImageViewCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ImageViewCreateInfoBuilder<'a> {
    pub fn insert_next<T: ImageViewCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ImageViewCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
    pub fn view_type(mut self, view_type: vk::ImageViewType) -> Self {
        self.inner.view_type = view_type;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn components(mut self, components: vk::ComponentMapping) -> Self {
        self.inner.components = components;
        self
    }
    pub fn subresource_range(mut self, subresource_range: vk::ImageSubresourceRange) -> Self {
        self.inner.subresource_range = subresource_range;
        self
    }
}
impl<'a> Deref for ImageViewCreateInfoBuilder<'a> {
    type Target = vk::ImageViewCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SparseBufferMemoryBindInfo {
    type Type = SparseBufferMemoryBindInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SparseBufferMemoryBindInfoBuilder<'a> {
    inner: vk::SparseBufferMemoryBindInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SparseBufferMemoryBindInfoBuilder<'a> {
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn p_binds(mut self, p_binds: &'a [vk::SparseMemoryBind]) -> Self {
        self.inner.bind_count = p_binds.len() as u32;
        self.inner.p_binds = p_binds.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for SparseBufferMemoryBindInfoBuilder<'a> {
    type Target = vk::SparseBufferMemoryBindInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SparseImageOpaqueMemoryBindInfo {
    type Type = SparseImageOpaqueMemoryBindInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SparseImageOpaqueMemoryBindInfoBuilder<'a> {
    inner: vk::SparseImageOpaqueMemoryBindInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SparseImageOpaqueMemoryBindInfoBuilder<'a> {
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
    pub fn p_binds(mut self, p_binds: &'a [vk::SparseMemoryBind]) -> Self {
        self.inner.bind_count = p_binds.len() as u32;
        self.inner.p_binds = p_binds.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for SparseImageOpaqueMemoryBindInfoBuilder<'a> {
    type Target = vk::SparseImageOpaqueMemoryBindInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SparseImageMemoryBindInfo {
    type Type = SparseImageMemoryBindInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SparseImageMemoryBindInfoBuilder<'a> {
    inner: vk::SparseImageMemoryBindInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SparseImageMemoryBindInfoBuilder<'a> {
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
    pub fn p_binds(mut self, p_binds: &'a [vk::SparseImageMemoryBind]) -> Self {
        self.inner.bind_count = p_binds.len() as u32;
        self.inner.p_binds = p_binds.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for SparseImageMemoryBindInfoBuilder<'a> {
    type Target = vk::SparseImageMemoryBindInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BindSparseInfo {
    type Type = BindSparseInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait BindSparseInfoNext {}
#[derive(Default)]
pub struct BindSparseInfoBuilder<'a> {
    inner: vk::BindSparseInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BindSparseInfoBuilder<'a> {
    pub fn insert_next<T: BindSparseInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphores(mut self, p_wait_semaphores: &'a [vk::Semaphore]) -> Self {
        self.inner.wait_semaphore_count = p_wait_semaphores.len() as u32;
        self.inner.p_wait_semaphores = p_wait_semaphores.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_buffer_binds(mut self, p_buffer_binds: &'a [vk::SparseBufferMemoryBindInfo]) -> Self {
        self.inner.buffer_bind_count = p_buffer_binds.len() as u32;
        self.inner.p_buffer_binds = p_buffer_binds.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_image_opaque_binds(mut self, p_image_opaque_binds: &'a [vk::SparseImageOpaqueMemoryBindInfo]) -> Self {
        self.inner.image_opaque_bind_count = p_image_opaque_binds.len() as u32;
        self.inner.p_image_opaque_binds = p_image_opaque_binds.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_image_binds(mut self, p_image_binds: &'a [vk::SparseImageMemoryBindInfo]) -> Self {
        self.inner.image_bind_count = p_image_binds.len() as u32;
        self.inner.p_image_binds = p_image_binds.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_signal_semaphores(mut self, p_signal_semaphores: &'a [vk::Semaphore]) -> Self {
        self.inner.signal_semaphore_count = p_signal_semaphores.len() as u32;
        self.inner.p_signal_semaphores = p_signal_semaphores.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for BindSparseInfoBuilder<'a> {
    type Target = vk::BindSparseInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ShaderModuleCreateInfo {
    type Type = ShaderModuleCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ShaderModuleCreateInfoNext {}
#[derive(Default)]
pub struct ShaderModuleCreateInfoBuilder<'a> {
    inner: vk::ShaderModuleCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ShaderModuleCreateInfoBuilder<'a> {
    pub fn insert_next<T: ShaderModuleCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ShaderModuleCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn code_size(mut self, code_size: usize) -> Self {
        self.inner.code_size = code_size;
        self
    }
    pub fn p_code(mut self, p_code: *const u32) -> Self {
        self.inner.p_code = p_code;
        self
    }
}
impl<'a> Deref for ShaderModuleCreateInfoBuilder<'a> {
    type Target = vk::ShaderModuleCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> PipelineShaderStageCreateInfoNext for ShaderModuleCreateInfoBuilder<'a> {}
impl PipelineShaderStageCreateInfoNext for vk::ShaderModuleCreateInfo {}
impl<'a> Builder<'a> for vk::DescriptorSetLayoutBinding {
    type Type = DescriptorSetLayoutBindingBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorSetLayoutBindingBuilder<'a> {
    inner: vk::DescriptorSetLayoutBinding,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DescriptorSetLayoutBindingBuilder<'a> {
    pub fn binding(mut self, binding: u32) -> Self {
        self.inner.binding = binding;
        self
    }
    pub fn descriptor_type(mut self, descriptor_type: vk::DescriptorType) -> Self {
        self.inner.descriptor_type = descriptor_type;
        self
    }
    pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.inner.descriptor_count = descriptor_count;
        self
    }
    pub fn p_immutable_samplers(mut self, p_immutable_samplers: &'a [vk::Sampler]) -> Self {
        self.inner.descriptor_count = p_immutable_samplers.len() as u32;
        self.inner.p_immutable_samplers = p_immutable_samplers.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn stage_flags(mut self, stage_flags: vk::ShaderStageFlags) -> Self {
        self.inner.stage_flags = stage_flags;
        self
    }
}
impl<'a> Deref for DescriptorSetLayoutBindingBuilder<'a> {
    type Target = vk::DescriptorSetLayoutBinding;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DescriptorSetLayoutCreateInfo {
    type Type = DescriptorSetLayoutCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DescriptorSetLayoutCreateInfoNext {}
#[derive(Default)]
pub struct DescriptorSetLayoutCreateInfoBuilder<'a> {
    inner: vk::DescriptorSetLayoutCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DescriptorSetLayoutCreateInfoBuilder<'a> {
    pub fn insert_next<T: DescriptorSetLayoutCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DescriptorSetLayoutCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_bindings(mut self, p_bindings: &'a [vk::DescriptorSetLayoutBinding]) -> Self {
        self.inner.binding_count = p_bindings.len() as u32;
        self.inner.p_bindings = p_bindings.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for DescriptorSetLayoutCreateInfoBuilder<'a> {
    type Target = vk::DescriptorSetLayoutCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DescriptorPoolCreateInfo {
    type Type = DescriptorPoolCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DescriptorPoolCreateInfoNext {}
#[derive(Default)]
pub struct DescriptorPoolCreateInfoBuilder<'a> {
    inner: vk::DescriptorPoolCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DescriptorPoolCreateInfoBuilder<'a> {
    pub fn insert_next<T: DescriptorPoolCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DescriptorPoolCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn max_sets(mut self, max_sets: u32) -> Self {
        self.inner.max_sets = max_sets;
        self
    }
    pub fn p_pool_sizes(mut self, p_pool_sizes: &'a [vk::DescriptorPoolSize]) -> Self {
        self.inner.pool_size_count = p_pool_sizes.len() as u32;
        self.inner.p_pool_sizes = p_pool_sizes.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for DescriptorPoolCreateInfoBuilder<'a> {
    type Target = vk::DescriptorPoolCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DescriptorSetAllocateInfo {
    type Type = DescriptorSetAllocateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DescriptorSetAllocateInfoNext {}
#[derive(Default)]
pub struct DescriptorSetAllocateInfoBuilder<'a> {
    inner: vk::DescriptorSetAllocateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DescriptorSetAllocateInfoBuilder<'a> {
    pub fn insert_next<T: DescriptorSetAllocateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn descriptor_pool(mut self, descriptor_pool: vk::DescriptorPool) -> Self {
        self.inner.descriptor_pool = Some(descriptor_pool);
        self
    }
    pub fn p_set_layouts(mut self, p_set_layouts: &'a [vk::DescriptorSetLayout]) -> Self {
        self.inner.descriptor_set_count = p_set_layouts.len() as u32;
        self.inner.p_set_layouts = p_set_layouts.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for DescriptorSetAllocateInfoBuilder<'a> {
    type Target = vk::DescriptorSetAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SpecializationInfo {
    type Type = SpecializationInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SpecializationInfoBuilder<'a> {
    inner: vk::SpecializationInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SpecializationInfoBuilder<'a> {
    pub fn p_map_entries(mut self, p_map_entries: &'a [vk::SpecializationMapEntry]) -> Self {
        self.inner.map_entry_count = p_map_entries.len() as u32;
        self.inner.p_map_entries = p_map_entries.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_data<T>(mut self, p_data: &'a [T]) -> Self {
        self.inner.data_size = mem::size_of_val(p_data) as usize;
        self.inner.p_data = p_data.first().map_or(ptr::null(), |s| s as *const _) as *const _;
        self
    }
}
impl<'a> Deref for SpecializationInfoBuilder<'a> {
    type Target = vk::SpecializationInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineShaderStageCreateInfo {
    type Type = PipelineShaderStageCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PipelineShaderStageCreateInfoNext {}
#[derive(Default)]
pub struct PipelineShaderStageCreateInfoBuilder<'a> {
    inner: vk::PipelineShaderStageCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineShaderStageCreateInfoBuilder<'a> {
    pub fn insert_next<T: PipelineShaderStageCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineShaderStageCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn stage(mut self, stage: vk::ShaderStageFlags) -> Self {
        self.inner.stage = stage;
        self
    }
    pub fn module(mut self, module: Option<vk::ShaderModule>) -> Self {
        self.inner.module = module;
        self
    }
    pub fn p_name(mut self, p_name: &'a CStr) -> Self {
        self.inner.p_name = p_name.as_ptr();
        self
    }
    pub fn p_specialization_info(mut self, p_specialization_info: Option<&'a vk::SpecializationInfo>) -> Self {
        self.inner.p_specialization_info = p_specialization_info.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for PipelineShaderStageCreateInfoBuilder<'a> {
    type Target = vk::PipelineShaderStageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ComputePipelineCreateInfo {
    type Type = ComputePipelineCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ComputePipelineCreateInfoNext {}
#[derive(Default)]
pub struct ComputePipelineCreateInfoBuilder<'a> {
    inner: vk::ComputePipelineCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ComputePipelineCreateInfoBuilder<'a> {
    pub fn insert_next<T: ComputePipelineCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn stage(mut self, stage: vk::PipelineShaderStageCreateInfo) -> Self {
        self.inner.stage = stage;
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = Some(layout);
        self
    }
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: Option<vk::Pipeline>) -> Self {
        self.inner.base_pipeline_handle = base_pipeline_handle;
        self
    }
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.inner.base_pipeline_index = base_pipeline_index;
        self
    }
}
impl<'a> Deref for ComputePipelineCreateInfoBuilder<'a> {
    type Target = vk::ComputePipelineCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineVertexInputStateCreateInfo {
    type Type = PipelineVertexInputStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PipelineVertexInputStateCreateInfoNext {}
#[derive(Default)]
pub struct PipelineVertexInputStateCreateInfoBuilder<'a> {
    inner: vk::PipelineVertexInputStateCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineVertexInputStateCreateInfoBuilder<'a> {
    pub fn insert_next<T: PipelineVertexInputStateCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineVertexInputStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_vertex_binding_descriptions(
        mut self,
        p_vertex_binding_descriptions: &'a [vk::VertexInputBindingDescription],
    ) -> Self {
        self.inner.vertex_binding_description_count = p_vertex_binding_descriptions.len() as u32;
        self.inner.p_vertex_binding_descriptions = p_vertex_binding_descriptions
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_vertex_attribute_descriptions(
        mut self,
        p_vertex_attribute_descriptions: &'a [vk::VertexInputAttributeDescription],
    ) -> Self {
        self.inner.vertex_attribute_description_count = p_vertex_attribute_descriptions.len() as u32;
        self.inner.p_vertex_attribute_descriptions = p_vertex_attribute_descriptions
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PipelineVertexInputStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineVertexInputStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineInputAssemblyStateCreateInfo {
    type Type = PipelineInputAssemblyStateCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineInputAssemblyStateCreateInfoBuilder {
    inner: vk::PipelineInputAssemblyStateCreateInfo,
}
impl PipelineInputAssemblyStateCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineInputAssemblyStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn topology(mut self, topology: vk::PrimitiveTopology) -> Self {
        self.inner.topology = topology;
        self
    }
    pub fn primitive_restart_enable(mut self, primitive_restart_enable: bool) -> Self {
        self.inner.primitive_restart_enable = if primitive_restart_enable { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PipelineInputAssemblyStateCreateInfoBuilder {
    type Target = vk::PipelineInputAssemblyStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineTessellationStateCreateInfo {
    type Type = PipelineTessellationStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PipelineTessellationStateCreateInfoNext {}
#[derive(Default)]
pub struct PipelineTessellationStateCreateInfoBuilder<'a> {
    inner: vk::PipelineTessellationStateCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineTessellationStateCreateInfoBuilder<'a> {
    pub fn insert_next<T: PipelineTessellationStateCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineTessellationStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn patch_control_points(mut self, patch_control_points: u32) -> Self {
        self.inner.patch_control_points = patch_control_points;
        self
    }
}
impl<'a> Deref for PipelineTessellationStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineTessellationStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineViewportStateCreateInfo {
    type Type = PipelineViewportStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PipelineViewportStateCreateInfoNext {}
#[derive(Default)]
pub struct PipelineViewportStateCreateInfoBuilder<'a> {
    inner: vk::PipelineViewportStateCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineViewportStateCreateInfoBuilder<'a> {
    pub fn insert_next<T: PipelineViewportStateCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineViewportStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn viewport_count(mut self, viewport_count: u32) -> Self {
        self.inner.viewport_count = viewport_count;
        self
    }
    pub fn p_viewports(mut self, p_viewports: &'a [vk::Viewport]) -> Self {
        self.inner.viewport_count = p_viewports.len() as u32;
        self.inner.p_viewports = p_viewports.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn scissor_count(mut self, scissor_count: u32) -> Self {
        self.inner.scissor_count = scissor_count;
        self
    }
    pub fn p_scissors(mut self, p_scissors: &'a [vk::Rect2D]) -> Self {
        self.inner.scissor_count = p_scissors.len() as u32;
        self.inner.p_scissors = p_scissors.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PipelineViewportStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineViewportStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineRasterizationStateCreateInfo {
    type Type = PipelineRasterizationStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PipelineRasterizationStateCreateInfoNext {}
#[derive(Default)]
pub struct PipelineRasterizationStateCreateInfoBuilder<'a> {
    inner: vk::PipelineRasterizationStateCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineRasterizationStateCreateInfoBuilder<'a> {
    pub fn insert_next<T: PipelineRasterizationStateCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineRasterizationStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn depth_clamp_enable(mut self, depth_clamp_enable: bool) -> Self {
        self.inner.depth_clamp_enable = if depth_clamp_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn rasterizer_discard_enable(mut self, rasterizer_discard_enable: bool) -> Self {
        self.inner.rasterizer_discard_enable = if rasterizer_discard_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn polygon_mode(mut self, polygon_mode: vk::PolygonMode) -> Self {
        self.inner.polygon_mode = polygon_mode;
        self
    }
    pub fn cull_mode(mut self, cull_mode: vk::CullModeFlags) -> Self {
        self.inner.cull_mode = cull_mode;
        self
    }
    pub fn front_face(mut self, front_face: vk::FrontFace) -> Self {
        self.inner.front_face = front_face;
        self
    }
    pub fn depth_bias_enable(mut self, depth_bias_enable: bool) -> Self {
        self.inner.depth_bias_enable = if depth_bias_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn depth_bias_constant_factor(mut self, depth_bias_constant_factor: f32) -> Self {
        self.inner.depth_bias_constant_factor = depth_bias_constant_factor;
        self
    }
    pub fn depth_bias_clamp(mut self, depth_bias_clamp: f32) -> Self {
        self.inner.depth_bias_clamp = depth_bias_clamp;
        self
    }
    pub fn depth_bias_slope_factor(mut self, depth_bias_slope_factor: f32) -> Self {
        self.inner.depth_bias_slope_factor = depth_bias_slope_factor;
        self
    }
    pub fn line_width(mut self, line_width: f32) -> Self {
        self.inner.line_width = line_width;
        self
    }
}
impl<'a> Deref for PipelineRasterizationStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineRasterizationStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineMultisampleStateCreateInfo {
    type Type = PipelineMultisampleStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PipelineMultisampleStateCreateInfoNext {}
#[derive(Default)]
pub struct PipelineMultisampleStateCreateInfoBuilder<'a> {
    inner: vk::PipelineMultisampleStateCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineMultisampleStateCreateInfoBuilder<'a> {
    pub fn insert_next<T: PipelineMultisampleStateCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineMultisampleStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn rasterization_samples(mut self, rasterization_samples: vk::SampleCountFlags) -> Self {
        self.inner.rasterization_samples = rasterization_samples;
        self
    }
    pub fn sample_shading_enable(mut self, sample_shading_enable: bool) -> Self {
        self.inner.sample_shading_enable = if sample_shading_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn min_sample_shading(mut self, min_sample_shading: f32) -> Self {
        self.inner.min_sample_shading = min_sample_shading;
        self
    }
    pub fn p_sample_mask(mut self, p_sample_mask: *const vk::SampleMask) -> Self {
        self.inner.p_sample_mask = p_sample_mask;
        self
    }
    pub fn alpha_to_coverage_enable(mut self, alpha_to_coverage_enable: bool) -> Self {
        self.inner.alpha_to_coverage_enable = if alpha_to_coverage_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn alpha_to_one_enable(mut self, alpha_to_one_enable: bool) -> Self {
        self.inner.alpha_to_one_enable = if alpha_to_one_enable { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PipelineMultisampleStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineMultisampleStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineColorBlendStateCreateInfo {
    type Type = PipelineColorBlendStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PipelineColorBlendStateCreateInfoNext {}
#[derive(Default)]
pub struct PipelineColorBlendStateCreateInfoBuilder<'a> {
    inner: vk::PipelineColorBlendStateCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineColorBlendStateCreateInfoBuilder<'a> {
    pub fn insert_next<T: PipelineColorBlendStateCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineColorBlendStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn logic_op_enable(mut self, logic_op_enable: bool) -> Self {
        self.inner.logic_op_enable = if logic_op_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn logic_op(mut self, logic_op: vk::LogicOp) -> Self {
        self.inner.logic_op = logic_op;
        self
    }
    pub fn p_attachments(mut self, p_attachments: &'a [vk::PipelineColorBlendAttachmentState]) -> Self {
        self.inner.attachment_count = p_attachments.len() as u32;
        self.inner.p_attachments = p_attachments.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PipelineColorBlendStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineColorBlendStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineDynamicStateCreateInfo {
    type Type = PipelineDynamicStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineDynamicStateCreateInfoBuilder<'a> {
    inner: vk::PipelineDynamicStateCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineDynamicStateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineDynamicStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_dynamic_states(mut self, p_dynamic_states: &'a [vk::DynamicState]) -> Self {
        self.inner.dynamic_state_count = p_dynamic_states.len() as u32;
        self.inner.p_dynamic_states = p_dynamic_states.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PipelineDynamicStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineDynamicStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineDepthStencilStateCreateInfo {
    type Type = PipelineDepthStencilStateCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineDepthStencilStateCreateInfoBuilder {
    inner: vk::PipelineDepthStencilStateCreateInfo,
}
impl PipelineDepthStencilStateCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineDepthStencilStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn depth_test_enable(mut self, depth_test_enable: bool) -> Self {
        self.inner.depth_test_enable = if depth_test_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn depth_write_enable(mut self, depth_write_enable: bool) -> Self {
        self.inner.depth_write_enable = if depth_write_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn depth_compare_op(mut self, depth_compare_op: vk::CompareOp) -> Self {
        self.inner.depth_compare_op = depth_compare_op;
        self
    }
    pub fn depth_bounds_test_enable(mut self, depth_bounds_test_enable: bool) -> Self {
        self.inner.depth_bounds_test_enable = if depth_bounds_test_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn stencil_test_enable(mut self, stencil_test_enable: bool) -> Self {
        self.inner.stencil_test_enable = if stencil_test_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn front(mut self, front: vk::StencilOpState) -> Self {
        self.inner.front = front;
        self
    }
    pub fn back(mut self, back: vk::StencilOpState) -> Self {
        self.inner.back = back;
        self
    }
    pub fn min_depth_bounds(mut self, min_depth_bounds: f32) -> Self {
        self.inner.min_depth_bounds = min_depth_bounds;
        self
    }
    pub fn max_depth_bounds(mut self, max_depth_bounds: f32) -> Self {
        self.inner.max_depth_bounds = max_depth_bounds;
        self
    }
}
impl Deref for PipelineDepthStencilStateCreateInfoBuilder {
    type Target = vk::PipelineDepthStencilStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::GraphicsPipelineCreateInfo {
    type Type = GraphicsPipelineCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait GraphicsPipelineCreateInfoNext {}
#[derive(Default)]
pub struct GraphicsPipelineCreateInfoBuilder<'a> {
    inner: vk::GraphicsPipelineCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> GraphicsPipelineCreateInfoBuilder<'a> {
    pub fn insert_next<T: GraphicsPipelineCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn stage_count(mut self, stage_count: u32) -> Self {
        self.inner.stage_count = stage_count;
        self
    }
    pub fn p_stages(mut self, p_stages: &'a [vk::PipelineShaderStageCreateInfo]) -> Self {
        self.inner.stage_count = p_stages.len() as u32;
        self.inner.p_stages = p_stages.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_vertex_input_state(
        mut self,
        p_vertex_input_state: Option<&'a vk::PipelineVertexInputStateCreateInfo>,
    ) -> Self {
        self.inner.p_vertex_input_state = p_vertex_input_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_input_assembly_state(
        mut self,
        p_input_assembly_state: Option<&'a vk::PipelineInputAssemblyStateCreateInfo>,
    ) -> Self {
        self.inner.p_input_assembly_state = p_input_assembly_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_tessellation_state(
        mut self,
        p_tessellation_state: Option<&'a vk::PipelineTessellationStateCreateInfo>,
    ) -> Self {
        self.inner.p_tessellation_state = p_tessellation_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_viewport_state(mut self, p_viewport_state: Option<&'a vk::PipelineViewportStateCreateInfo>) -> Self {
        self.inner.p_viewport_state = p_viewport_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_rasterization_state(
        mut self,
        p_rasterization_state: Option<&'a vk::PipelineRasterizationStateCreateInfo>,
    ) -> Self {
        self.inner.p_rasterization_state = p_rasterization_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_multisample_state(
        mut self,
        p_multisample_state: Option<&'a vk::PipelineMultisampleStateCreateInfo>,
    ) -> Self {
        self.inner.p_multisample_state = p_multisample_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_depth_stencil_state(
        mut self,
        p_depth_stencil_state: Option<&'a vk::PipelineDepthStencilStateCreateInfo>,
    ) -> Self {
        self.inner.p_depth_stencil_state = p_depth_stencil_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_color_blend_state(
        mut self,
        p_color_blend_state: Option<&'a vk::PipelineColorBlendStateCreateInfo>,
    ) -> Self {
        self.inner.p_color_blend_state = p_color_blend_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_dynamic_state(mut self, p_dynamic_state: Option<&'a vk::PipelineDynamicStateCreateInfo>) -> Self {
        self.inner.p_dynamic_state = p_dynamic_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn layout(mut self, layout: Option<vk::PipelineLayout>) -> Self {
        self.inner.layout = layout;
        self
    }
    pub fn render_pass(mut self, render_pass: Option<vk::RenderPass>) -> Self {
        self.inner.render_pass = render_pass;
        self
    }
    pub fn subpass(mut self, subpass: u32) -> Self {
        self.inner.subpass = subpass;
        self
    }
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: Option<vk::Pipeline>) -> Self {
        self.inner.base_pipeline_handle = base_pipeline_handle;
        self
    }
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.inner.base_pipeline_index = base_pipeline_index;
        self
    }
}
impl<'a> Deref for GraphicsPipelineCreateInfoBuilder<'a> {
    type Target = vk::GraphicsPipelineCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineCacheCreateInfo {
    type Type = PipelineCacheCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCacheCreateInfoBuilder<'a> {
    inner: vk::PipelineCacheCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineCacheCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCacheCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_initial_data<T>(mut self, p_initial_data: &'a [T]) -> Self {
        self.inner.initial_data_size = mem::size_of_val(p_initial_data) as usize;
        self.inner.p_initial_data = p_initial_data.first().map_or(ptr::null(), |s| s as *const _) as *const _;
        self
    }
}
impl<'a> Deref for PipelineCacheCreateInfoBuilder<'a> {
    type Target = vk::PipelineCacheCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineLayoutCreateInfo {
    type Type = PipelineLayoutCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineLayoutCreateInfoBuilder<'a> {
    inner: vk::PipelineLayoutCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineLayoutCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineLayoutCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_set_layouts(mut self, p_set_layouts: &'a [vk::DescriptorSetLayout]) -> Self {
        self.inner.set_layout_count = p_set_layouts.len() as u32;
        self.inner.p_set_layouts = p_set_layouts.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_push_constant_ranges(mut self, p_push_constant_ranges: &'a [vk::PushConstantRange]) -> Self {
        self.inner.push_constant_range_count = p_push_constant_ranges.len() as u32;
        self.inner.p_push_constant_ranges = p_push_constant_ranges.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PipelineLayoutCreateInfoBuilder<'a> {
    type Target = vk::PipelineLayoutCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SamplerCreateInfo {
    type Type = SamplerCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SamplerCreateInfoNext {}
#[derive(Default)]
pub struct SamplerCreateInfoBuilder<'a> {
    inner: vk::SamplerCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SamplerCreateInfoBuilder<'a> {
    pub fn insert_next<T: SamplerCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SamplerCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn mag_filter(mut self, mag_filter: vk::Filter) -> Self {
        self.inner.mag_filter = mag_filter;
        self
    }
    pub fn min_filter(mut self, min_filter: vk::Filter) -> Self {
        self.inner.min_filter = min_filter;
        self
    }
    pub fn mipmap_mode(mut self, mipmap_mode: vk::SamplerMipmapMode) -> Self {
        self.inner.mipmap_mode = mipmap_mode;
        self
    }
    pub fn address_mode_u(mut self, address_mode_u: vk::SamplerAddressMode) -> Self {
        self.inner.address_mode_u = address_mode_u;
        self
    }
    pub fn address_mode_v(mut self, address_mode_v: vk::SamplerAddressMode) -> Self {
        self.inner.address_mode_v = address_mode_v;
        self
    }
    pub fn address_mode_w(mut self, address_mode_w: vk::SamplerAddressMode) -> Self {
        self.inner.address_mode_w = address_mode_w;
        self
    }
    pub fn mip_lod_bias(mut self, mip_lod_bias: f32) -> Self {
        self.inner.mip_lod_bias = mip_lod_bias;
        self
    }
    pub fn anisotropy_enable(mut self, anisotropy_enable: bool) -> Self {
        self.inner.anisotropy_enable = if anisotropy_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn max_anisotropy(mut self, max_anisotropy: f32) -> Self {
        self.inner.max_anisotropy = max_anisotropy;
        self
    }
    pub fn compare_enable(mut self, compare_enable: bool) -> Self {
        self.inner.compare_enable = if compare_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn compare_op(mut self, compare_op: vk::CompareOp) -> Self {
        self.inner.compare_op = compare_op;
        self
    }
    pub fn min_lod(mut self, min_lod: f32) -> Self {
        self.inner.min_lod = min_lod;
        self
    }
    pub fn max_lod(mut self, max_lod: f32) -> Self {
        self.inner.max_lod = max_lod;
        self
    }
    pub fn border_color(mut self, border_color: vk::BorderColor) -> Self {
        self.inner.border_color = border_color;
        self
    }
    pub fn unnormalized_coordinates(mut self, unnormalized_coordinates: bool) -> Self {
        self.inner.unnormalized_coordinates = if unnormalized_coordinates { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for SamplerCreateInfoBuilder<'a> {
    type Target = vk::SamplerCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CommandPoolCreateInfo {
    type Type = CommandPoolCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandPoolCreateInfoBuilder {
    inner: vk::CommandPoolCreateInfo,
}
impl CommandPoolCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::CommandPoolCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.inner.queue_family_index = queue_family_index;
        self
    }
}
impl Deref for CommandPoolCreateInfoBuilder {
    type Target = vk::CommandPoolCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CommandBufferAllocateInfo {
    type Type = CommandBufferAllocateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandBufferAllocateInfoBuilder {
    inner: vk::CommandBufferAllocateInfo,
}
impl CommandBufferAllocateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn command_pool(mut self, command_pool: vk::CommandPool) -> Self {
        self.inner.command_pool = Some(command_pool);
        self
    }
    pub fn level(mut self, level: vk::CommandBufferLevel) -> Self {
        self.inner.level = level;
        self
    }
    pub fn command_buffer_count(mut self, command_buffer_count: u32) -> Self {
        self.inner.command_buffer_count = command_buffer_count;
        self
    }
}
impl Deref for CommandBufferAllocateInfoBuilder {
    type Target = vk::CommandBufferAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CommandBufferInheritanceInfo {
    type Type = CommandBufferInheritanceInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait CommandBufferInheritanceInfoNext {}
#[derive(Default)]
pub struct CommandBufferInheritanceInfoBuilder<'a> {
    inner: vk::CommandBufferInheritanceInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CommandBufferInheritanceInfoBuilder<'a> {
    pub fn insert_next<T: CommandBufferInheritanceInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn render_pass(mut self, render_pass: Option<vk::RenderPass>) -> Self {
        self.inner.render_pass = render_pass;
        self
    }
    pub fn subpass(mut self, subpass: u32) -> Self {
        self.inner.subpass = subpass;
        self
    }
    pub fn framebuffer(mut self, framebuffer: Option<vk::Framebuffer>) -> Self {
        self.inner.framebuffer = framebuffer;
        self
    }
    pub fn occlusion_query_enable(mut self, occlusion_query_enable: bool) -> Self {
        self.inner.occlusion_query_enable = if occlusion_query_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn query_flags(mut self, query_flags: vk::QueryControlFlags) -> Self {
        self.inner.query_flags = query_flags;
        self
    }
    pub fn pipeline_statistics(mut self, pipeline_statistics: vk::QueryPipelineStatisticFlags) -> Self {
        self.inner.pipeline_statistics = pipeline_statistics;
        self
    }
}
impl<'a> Deref for CommandBufferInheritanceInfoBuilder<'a> {
    type Target = vk::CommandBufferInheritanceInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CommandBufferBeginInfo {
    type Type = CommandBufferBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait CommandBufferBeginInfoNext {}
#[derive(Default)]
pub struct CommandBufferBeginInfoBuilder<'a> {
    inner: vk::CommandBufferBeginInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CommandBufferBeginInfoBuilder<'a> {
    pub fn insert_next<T: CommandBufferBeginInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::CommandBufferUsageFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_inheritance_info(mut self, p_inheritance_info: Option<&'a vk::CommandBufferInheritanceInfo>) -> Self {
        self.inner.p_inheritance_info = p_inheritance_info.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for CommandBufferBeginInfoBuilder<'a> {
    type Target = vk::CommandBufferBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RenderPassBeginInfo {
    type Type = RenderPassBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait RenderPassBeginInfoNext {}
#[derive(Default)]
pub struct RenderPassBeginInfoBuilder<'a> {
    inner: vk::RenderPassBeginInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RenderPassBeginInfoBuilder<'a> {
    pub fn insert_next<T: RenderPassBeginInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn render_pass(mut self, render_pass: vk::RenderPass) -> Self {
        self.inner.render_pass = Some(render_pass);
        self
    }
    pub fn framebuffer(mut self, framebuffer: vk::Framebuffer) -> Self {
        self.inner.framebuffer = Some(framebuffer);
        self
    }
    pub fn render_area(mut self, render_area: vk::Rect2D) -> Self {
        self.inner.render_area = render_area;
        self
    }
    pub fn p_clear_values(mut self, p_clear_values: &'a [vk::ClearValue]) -> Self {
        self.inner.clear_value_count = p_clear_values.len() as u32;
        self.inner.p_clear_values = p_clear_values.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for RenderPassBeginInfoBuilder<'a> {
    type Target = vk::RenderPassBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SubpassDescription {
    type Type = SubpassDescriptionBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubpassDescriptionBuilder<'a> {
    inner: vk::SubpassDescription,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SubpassDescriptionBuilder<'a> {
    pub fn flags(mut self, flags: vk::SubpassDescriptionFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn p_input_attachments(mut self, p_input_attachments: &'a [vk::AttachmentReference]) -> Self {
        self.inner.input_attachment_count = p_input_attachments.len() as u32;
        self.inner.p_input_attachments = p_input_attachments.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_color_attachments(
        mut self,
        p_color_attachments: &'a [vk::AttachmentReference],
        p_resolve_attachments: Option<&'a [vk::AttachmentReference]>,
    ) -> Self {
        self.inner.color_attachment_count = p_color_attachments.len() as u32;
        if let Some(s) = p_resolve_attachments {
            assert_eq!(self.inner.color_attachment_count, s.len() as u32);
        }
        self.inner.p_color_attachments = p_color_attachments.first().map_or(ptr::null(), |s| s as *const _);
        self.inner.p_resolve_attachments = p_resolve_attachments
            .and_then(|s| s.first())
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_depth_stencil_attachment(
        mut self,
        p_depth_stencil_attachment: Option<&'a vk::AttachmentReference>,
    ) -> Self {
        self.inner.p_depth_stencil_attachment = p_depth_stencil_attachment.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_preserve_attachments(mut self, p_preserve_attachments: &'a [u32]) -> Self {
        self.inner.preserve_attachment_count = p_preserve_attachments.len() as u32;
        self.inner.p_preserve_attachments = p_preserve_attachments.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for SubpassDescriptionBuilder<'a> {
    type Target = vk::SubpassDescription;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RenderPassCreateInfo {
    type Type = RenderPassCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait RenderPassCreateInfoNext {}
#[derive(Default)]
pub struct RenderPassCreateInfoBuilder<'a> {
    inner: vk::RenderPassCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RenderPassCreateInfoBuilder<'a> {
    pub fn insert_next<T: RenderPassCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::RenderPassCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_attachments(mut self, p_attachments: &'a [vk::AttachmentDescription]) -> Self {
        self.inner.attachment_count = p_attachments.len() as u32;
        self.inner.p_attachments = p_attachments.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_subpasses(mut self, p_subpasses: &'a [vk::SubpassDescription]) -> Self {
        self.inner.subpass_count = p_subpasses.len() as u32;
        self.inner.p_subpasses = p_subpasses.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_dependencies(mut self, p_dependencies: &'a [vk::SubpassDependency]) -> Self {
        self.inner.dependency_count = p_dependencies.len() as u32;
        self.inner.p_dependencies = p_dependencies.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for RenderPassCreateInfoBuilder<'a> {
    type Target = vk::RenderPassCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::EventCreateInfo {
    type Type = EventCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct EventCreateInfoBuilder {
    inner: vk::EventCreateInfo,
}
impl EventCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::EventCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for EventCreateInfoBuilder {
    type Target = vk::EventCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::FenceCreateInfo {
    type Type = FenceCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait FenceCreateInfoNext {}
#[derive(Default)]
pub struct FenceCreateInfoBuilder<'a> {
    inner: vk::FenceCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> FenceCreateInfoBuilder<'a> {
    pub fn insert_next<T: FenceCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::FenceCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl<'a> Deref for FenceCreateInfoBuilder<'a> {
    type Target = vk::FenceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SemaphoreCreateInfo {
    type Type = SemaphoreCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SemaphoreCreateInfoNext {}
#[derive(Default)]
pub struct SemaphoreCreateInfoBuilder<'a> {
    inner: vk::SemaphoreCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SemaphoreCreateInfoBuilder<'a> {
    pub fn insert_next<T: SemaphoreCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SemaphoreCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl<'a> Deref for SemaphoreCreateInfoBuilder<'a> {
    type Target = vk::SemaphoreCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::QueryPoolCreateInfo {
    type Type = QueryPoolCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait QueryPoolCreateInfoNext {}
#[derive(Default)]
pub struct QueryPoolCreateInfoBuilder<'a> {
    inner: vk::QueryPoolCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> QueryPoolCreateInfoBuilder<'a> {
    pub fn insert_next<T: QueryPoolCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::QueryPoolCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn query_type(mut self, query_type: vk::QueryType) -> Self {
        self.inner.query_type = query_type;
        self
    }
    pub fn query_count(mut self, query_count: u32) -> Self {
        self.inner.query_count = query_count;
        self
    }
    pub fn pipeline_statistics(mut self, pipeline_statistics: vk::QueryPipelineStatisticFlags) -> Self {
        self.inner.pipeline_statistics = pipeline_statistics;
        self
    }
}
impl<'a> Deref for QueryPoolCreateInfoBuilder<'a> {
    type Target = vk::QueryPoolCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::FramebufferCreateInfo {
    type Type = FramebufferCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait FramebufferCreateInfoNext {}
#[derive(Default)]
pub struct FramebufferCreateInfoBuilder<'a> {
    inner: vk::FramebufferCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> FramebufferCreateInfoBuilder<'a> {
    pub fn insert_next<T: FramebufferCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::FramebufferCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn render_pass(mut self, render_pass: vk::RenderPass) -> Self {
        self.inner.render_pass = Some(render_pass);
        self
    }
    pub fn p_attachments(mut self, p_attachments: &'a [vk::ImageView]) -> Self {
        self.inner.attachment_count = p_attachments.len() as u32;
        self.inner.p_attachments = p_attachments.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn width(mut self, width: u32) -> Self {
        self.inner.width = width;
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.inner.height = height;
        self
    }
    pub fn layers(mut self, layers: u32) -> Self {
        self.inner.layers = layers;
        self
    }
}
impl<'a> Deref for FramebufferCreateInfoBuilder<'a> {
    type Target = vk::FramebufferCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SubmitInfo {
    type Type = SubmitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SubmitInfoNext {}
#[derive(Default)]
pub struct SubmitInfoBuilder<'a> {
    inner: vk::SubmitInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SubmitInfoBuilder<'a> {
    pub fn insert_next<T: SubmitInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphores(
        mut self,
        p_wait_semaphores: &'a [vk::Semaphore],
        p_wait_dst_stage_mask: &'a [vk::PipelineStageFlags],
    ) -> Self {
        self.inner.wait_semaphore_count = p_wait_semaphores.len() as u32;
        assert_eq!(self.inner.wait_semaphore_count, p_wait_dst_stage_mask.len() as u32);
        self.inner.p_wait_semaphores = p_wait_semaphores.first().map_or(ptr::null(), |s| s as *const _);
        self.inner.p_wait_dst_stage_mask = p_wait_dst_stage_mask.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_command_buffers(mut self, p_command_buffers: &'a [vk::CommandBuffer]) -> Self {
        self.inner.command_buffer_count = p_command_buffers.len() as u32;
        self.inner.p_command_buffers = p_command_buffers.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_signal_semaphores(mut self, p_signal_semaphores: &'a [vk::Semaphore]) -> Self {
        self.inner.signal_semaphore_count = p_signal_semaphores.len() as u32;
        self.inner.p_signal_semaphores = p_signal_semaphores.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for SubmitInfoBuilder<'a> {
    type Target = vk::SubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DisplayModeCreateInfoKHR {
    type Type = DisplayModeCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplayModeCreateInfoKHRBuilder {
    inner: vk::DisplayModeCreateInfoKHR,
}
impl DisplayModeCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DisplayModeCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn parameters(mut self, parameters: vk::DisplayModeParametersKHR) -> Self {
        self.inner.parameters = parameters;
        self
    }
}
impl Deref for DisplayModeCreateInfoKHRBuilder {
    type Target = vk::DisplayModeCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DisplaySurfaceCreateInfoKHR {
    type Type = DisplaySurfaceCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplaySurfaceCreateInfoKHRBuilder {
    inner: vk::DisplaySurfaceCreateInfoKHR,
}
impl DisplaySurfaceCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DisplaySurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn display_mode(mut self, display_mode: vk::DisplayModeKHR) -> Self {
        self.inner.display_mode = Some(display_mode);
        self
    }
    pub fn plane_index(mut self, plane_index: u32) -> Self {
        self.inner.plane_index = plane_index;
        self
    }
    pub fn plane_stack_index(mut self, plane_stack_index: u32) -> Self {
        self.inner.plane_stack_index = plane_stack_index;
        self
    }
    pub fn transform(mut self, transform: vk::SurfaceTransformFlagsKHR) -> Self {
        self.inner.transform = transform;
        self
    }
    pub fn global_alpha(mut self, global_alpha: f32) -> Self {
        self.inner.global_alpha = global_alpha;
        self
    }
    pub fn alpha_mode(mut self, alpha_mode: vk::DisplayPlaneAlphaFlagsKHR) -> Self {
        self.inner.alpha_mode = alpha_mode;
        self
    }
    pub fn image_extent(mut self, image_extent: vk::Extent2D) -> Self {
        self.inner.image_extent = image_extent;
        self
    }
}
impl Deref for DisplaySurfaceCreateInfoKHRBuilder {
    type Target = vk::DisplaySurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DisplayPresentInfoKHR {
    type Type = DisplayPresentInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplayPresentInfoKHRBuilder {
    inner: vk::DisplayPresentInfoKHR,
}
impl DisplayPresentInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_rect(mut self, src_rect: vk::Rect2D) -> Self {
        self.inner.src_rect = src_rect;
        self
    }
    pub fn dst_rect(mut self, dst_rect: vk::Rect2D) -> Self {
        self.inner.dst_rect = dst_rect;
        self
    }
    pub fn persistent(mut self, persistent: bool) -> Self {
        self.inner.persistent = if persistent { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for DisplayPresentInfoKHRBuilder {
    type Target = vk::DisplayPresentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PresentInfoKHRNext for DisplayPresentInfoKHRBuilder {}
impl PresentInfoKHRNext for vk::DisplayPresentInfoKHR {}
impl Builder<'_> for vk::AndroidSurfaceCreateInfoKHR {
    type Type = AndroidSurfaceCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AndroidSurfaceCreateInfoKHRBuilder {
    inner: vk::AndroidSurfaceCreateInfoKHR,
}
impl AndroidSurfaceCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::AndroidSurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn window(mut self, window: *mut vk::ANativeWindow) -> Self {
        self.inner.window = window;
        self
    }
}
impl Deref for AndroidSurfaceCreateInfoKHRBuilder {
    type Target = vk::AndroidSurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ViSurfaceCreateInfoNN {
    type Type = ViSurfaceCreateInfoNNBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ViSurfaceCreateInfoNNBuilder {
    inner: vk::ViSurfaceCreateInfoNN,
}
impl ViSurfaceCreateInfoNNBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ViSurfaceCreateFlagsNN) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn window(mut self, window: *mut c_void) -> Self {
        self.inner.window = window;
        self
    }
}
impl Deref for ViSurfaceCreateInfoNNBuilder {
    type Target = vk::ViSurfaceCreateInfoNN;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::WaylandSurfaceCreateInfoKHR {
    type Type = WaylandSurfaceCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct WaylandSurfaceCreateInfoKHRBuilder {
    inner: vk::WaylandSurfaceCreateInfoKHR,
}
impl WaylandSurfaceCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::WaylandSurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn display(mut self, display: *mut vk::wl_display) -> Self {
        self.inner.display = display;
        self
    }
    pub fn surface(mut self, surface: *mut vk::wl_surface) -> Self {
        self.inner.surface = surface;
        self
    }
}
impl Deref for WaylandSurfaceCreateInfoKHRBuilder {
    type Target = vk::WaylandSurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::Win32SurfaceCreateInfoKHR {
    type Type = Win32SurfaceCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct Win32SurfaceCreateInfoKHRBuilder {
    inner: vk::Win32SurfaceCreateInfoKHR,
}
impl Win32SurfaceCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::Win32SurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn hinstance(mut self, hinstance: vk::HINSTANCE) -> Self {
        self.inner.hinstance = hinstance;
        self
    }
    pub fn hwnd(mut self, hwnd: vk::HWND) -> Self {
        self.inner.hwnd = hwnd;
        self
    }
}
impl Deref for Win32SurfaceCreateInfoKHRBuilder {
    type Target = vk::Win32SurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::XlibSurfaceCreateInfoKHR {
    type Type = XlibSurfaceCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct XlibSurfaceCreateInfoKHRBuilder {
    inner: vk::XlibSurfaceCreateInfoKHR,
}
impl XlibSurfaceCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::XlibSurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn dpy(mut self, dpy: *mut vk::Display) -> Self {
        self.inner.dpy = dpy;
        self
    }
    pub fn window(mut self, window: vk::Window) -> Self {
        self.inner.window = window;
        self
    }
}
impl Deref for XlibSurfaceCreateInfoKHRBuilder {
    type Target = vk::XlibSurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::XcbSurfaceCreateInfoKHR {
    type Type = XcbSurfaceCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct XcbSurfaceCreateInfoKHRBuilder {
    inner: vk::XcbSurfaceCreateInfoKHR,
}
impl XcbSurfaceCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::XcbSurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn connection(mut self, connection: *mut vk::xcb_connection_t) -> Self {
        self.inner.connection = connection;
        self
    }
    pub fn window(mut self, window: vk::xcb_window_t) -> Self {
        self.inner.window = window;
        self
    }
}
impl Deref for XcbSurfaceCreateInfoKHRBuilder {
    type Target = vk::XcbSurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DirectFBSurfaceCreateInfoEXT {
    type Type = DirectFBSurfaceCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DirectFBSurfaceCreateInfoEXTBuilder {
    inner: vk::DirectFBSurfaceCreateInfoEXT,
}
impl DirectFBSurfaceCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DirectFBSurfaceCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn dfb(mut self, dfb: *mut vk::IDirectFB) -> Self {
        self.inner.dfb = dfb;
        self
    }
    pub fn surface(mut self, surface: *mut vk::IDirectFBSurface) -> Self {
        self.inner.surface = surface;
        self
    }
}
impl Deref for DirectFBSurfaceCreateInfoEXTBuilder {
    type Target = vk::DirectFBSurfaceCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImagePipeSurfaceCreateInfoFUCHSIA {
    type Type = ImagePipeSurfaceCreateInfoFUCHSIABuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImagePipeSurfaceCreateInfoFUCHSIABuilder {
    inner: vk::ImagePipeSurfaceCreateInfoFUCHSIA,
}
impl ImagePipeSurfaceCreateInfoFUCHSIABuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ImagePipeSurfaceCreateFlagsFUCHSIA) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn image_pipe_handle(mut self, image_pipe_handle: vk::zx_handle_t) -> Self {
        self.inner.image_pipe_handle = image_pipe_handle;
        self
    }
}
impl Deref for ImagePipeSurfaceCreateInfoFUCHSIABuilder {
    type Target = vk::ImagePipeSurfaceCreateInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SwapchainCreateInfoKHR {
    type Type = SwapchainCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SwapchainCreateInfoKHRNext {}
#[derive(Default)]
pub struct SwapchainCreateInfoKHRBuilder<'a> {
    inner: vk::SwapchainCreateInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SwapchainCreateInfoKHRBuilder<'a> {
    pub fn insert_next<T: SwapchainCreateInfoKHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SwapchainCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn surface(mut self, surface: vk::SurfaceKHR) -> Self {
        self.inner.surface = Some(surface);
        self
    }
    pub fn min_image_count(mut self, min_image_count: u32) -> Self {
        self.inner.min_image_count = min_image_count;
        self
    }
    pub fn image_format(mut self, image_format: vk::Format) -> Self {
        self.inner.image_format = image_format;
        self
    }
    pub fn image_color_space(mut self, image_color_space: vk::ColorSpaceKHR) -> Self {
        self.inner.image_color_space = image_color_space;
        self
    }
    pub fn image_extent(mut self, image_extent: vk::Extent2D) -> Self {
        self.inner.image_extent = image_extent;
        self
    }
    pub fn image_array_layers(mut self, image_array_layers: u32) -> Self {
        self.inner.image_array_layers = image_array_layers;
        self
    }
    pub fn image_usage(mut self, image_usage: vk::ImageUsageFlags) -> Self {
        self.inner.image_usage = image_usage;
        self
    }
    pub fn image_sharing_mode(mut self, image_sharing_mode: vk::SharingMode) -> Self {
        self.inner.image_sharing_mode = image_sharing_mode;
        self
    }
    pub fn p_queue_family_indices(mut self, p_queue_family_indices: &'a [u32]) -> Self {
        self.inner.queue_family_index_count = p_queue_family_indices.len() as u32;
        self.inner.p_queue_family_indices = p_queue_family_indices.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn pre_transform(mut self, pre_transform: vk::SurfaceTransformFlagsKHR) -> Self {
        self.inner.pre_transform = pre_transform;
        self
    }
    pub fn composite_alpha(mut self, composite_alpha: vk::CompositeAlphaFlagsKHR) -> Self {
        self.inner.composite_alpha = composite_alpha;
        self
    }
    pub fn present_mode(mut self, present_mode: vk::PresentModeKHR) -> Self {
        self.inner.present_mode = present_mode;
        self
    }
    pub fn clipped(mut self, clipped: bool) -> Self {
        self.inner.clipped = if clipped { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn old_swapchain(mut self, old_swapchain: Option<vk::SwapchainKHR>) -> Self {
        self.inner.old_swapchain = old_swapchain;
        self
    }
}
impl<'a> Deref for SwapchainCreateInfoKHRBuilder<'a> {
    type Target = vk::SwapchainCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PresentInfoKHR {
    type Type = PresentInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PresentInfoKHRNext {}
#[derive(Default)]
pub struct PresentInfoKHRBuilder<'a> {
    inner: vk::PresentInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PresentInfoKHRBuilder<'a> {
    pub fn insert_next<T: PresentInfoKHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphores(mut self, p_wait_semaphores: &'a [vk::Semaphore]) -> Self {
        self.inner.wait_semaphore_count = p_wait_semaphores.len() as u32;
        self.inner.p_wait_semaphores = p_wait_semaphores.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_swapchains(mut self, p_swapchains: &'a [vk::SwapchainKHR], p_image_indices: &'a [u32]) -> Self {
        self.inner.swapchain_count = p_swapchains.len() as u32;
        assert_eq!(self.inner.swapchain_count, p_image_indices.len() as u32);
        self.inner.p_swapchains = p_swapchains.first().map_or(ptr::null(), |s| s as *const _);
        self.inner.p_image_indices = p_image_indices.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_results(mut self, p_results: *mut vk::Result) -> Self {
        self.inner.p_results = p_results;
        self
    }
}
impl<'a> Deref for PresentInfoKHRBuilder<'a> {
    type Target = vk::PresentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DebugReportCallbackCreateInfoEXT {
    type Type = DebugReportCallbackCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugReportCallbackCreateInfoEXTBuilder {
    inner: vk::DebugReportCallbackCreateInfoEXT,
}
impl DebugReportCallbackCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DebugReportFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn pfn_callback(mut self, pfn_callback: vk::FnDebugReportCallbackEXT) -> Self {
        self.inner.pfn_callback = Some(pfn_callback);
        self
    }
    pub fn p_user_data(mut self, p_user_data: *mut c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
}
impl Deref for DebugReportCallbackCreateInfoEXTBuilder {
    type Target = vk::DebugReportCallbackCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl InstanceCreateInfoNext for DebugReportCallbackCreateInfoEXTBuilder {}
impl InstanceCreateInfoNext for vk::DebugReportCallbackCreateInfoEXT {}
impl<'a> Builder<'a> for vk::ValidationFlagsEXT {
    type Type = ValidationFlagsEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ValidationFlagsEXTBuilder<'a> {
    inner: vk::ValidationFlagsEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ValidationFlagsEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_disabled_validation_checks(mut self, p_disabled_validation_checks: &'a [vk::ValidationCheckEXT]) -> Self {
        self.inner.disabled_validation_check_count = p_disabled_validation_checks.len() as u32;
        self.inner.p_disabled_validation_checks = p_disabled_validation_checks
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for ValidationFlagsEXTBuilder<'a> {
    type Target = vk::ValidationFlagsEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> InstanceCreateInfoNext for ValidationFlagsEXTBuilder<'a> {}
impl InstanceCreateInfoNext for vk::ValidationFlagsEXT {}
impl<'a> Builder<'a> for vk::ValidationFeaturesEXT {
    type Type = ValidationFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ValidationFeaturesEXTBuilder<'a> {
    inner: vk::ValidationFeaturesEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ValidationFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_enabled_validation_features(
        mut self,
        p_enabled_validation_features: &'a [vk::ValidationFeatureEnableEXT],
    ) -> Self {
        self.inner.enabled_validation_feature_count = p_enabled_validation_features.len() as u32;
        self.inner.p_enabled_validation_features = p_enabled_validation_features
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_disabled_validation_features(
        mut self,
        p_disabled_validation_features: &'a [vk::ValidationFeatureDisableEXT],
    ) -> Self {
        self.inner.disabled_validation_feature_count = p_disabled_validation_features.len() as u32;
        self.inner.p_disabled_validation_features = p_disabled_validation_features
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for ValidationFeaturesEXTBuilder<'a> {
    type Target = vk::ValidationFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> InstanceCreateInfoNext for ValidationFeaturesEXTBuilder<'a> {}
impl InstanceCreateInfoNext for vk::ValidationFeaturesEXT {}
impl Builder<'_> for vk::PipelineRasterizationStateRasterizationOrderAMD {
    type Type = PipelineRasterizationStateRasterizationOrderAMDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationStateRasterizationOrderAMDBuilder {
    inner: vk::PipelineRasterizationStateRasterizationOrderAMD,
}
impl PipelineRasterizationStateRasterizationOrderAMDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn rasterization_order(mut self, rasterization_order: vk::RasterizationOrderAMD) -> Self {
        self.inner.rasterization_order = rasterization_order;
        self
    }
}
impl Deref for PipelineRasterizationStateRasterizationOrderAMDBuilder {
    type Target = vk::PipelineRasterizationStateRasterizationOrderAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineRasterizationStateCreateInfoNext for PipelineRasterizationStateRasterizationOrderAMDBuilder {}
impl PipelineRasterizationStateCreateInfoNext for vk::PipelineRasterizationStateRasterizationOrderAMD {}
impl<'a> Builder<'a> for vk::DebugMarkerObjectNameInfoEXT {
    type Type = DebugMarkerObjectNameInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugMarkerObjectNameInfoEXTBuilder<'a> {
    inner: vk::DebugMarkerObjectNameInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DebugMarkerObjectNameInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn object_type(mut self, object_type: vk::DebugReportObjectTypeEXT) -> Self {
        self.inner.object_type = object_type;
        self
    }
    pub fn object(mut self, object: u64) -> Self {
        self.inner.object = object;
        self
    }
    pub fn p_object_name(mut self, p_object_name: &'a CStr) -> Self {
        self.inner.p_object_name = p_object_name.as_ptr();
        self
    }
}
impl<'a> Deref for DebugMarkerObjectNameInfoEXTBuilder<'a> {
    type Target = vk::DebugMarkerObjectNameInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DebugMarkerObjectTagInfoEXT {
    type Type = DebugMarkerObjectTagInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugMarkerObjectTagInfoEXTBuilder<'a> {
    inner: vk::DebugMarkerObjectTagInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DebugMarkerObjectTagInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn object_type(mut self, object_type: vk::DebugReportObjectTypeEXT) -> Self {
        self.inner.object_type = object_type;
        self
    }
    pub fn object(mut self, object: u64) -> Self {
        self.inner.object = object;
        self
    }
    pub fn tag_name(mut self, tag_name: u64) -> Self {
        self.inner.tag_name = tag_name;
        self
    }
    pub fn p_tag<T>(mut self, p_tag: &'a [T]) -> Self {
        self.inner.tag_size = mem::size_of_val(p_tag) as usize;
        self.inner.p_tag = p_tag.first().map_or(ptr::null(), |s| s as *const _) as *const _;
        self
    }
}
impl<'a> Deref for DebugMarkerObjectTagInfoEXTBuilder<'a> {
    type Target = vk::DebugMarkerObjectTagInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DebugMarkerMarkerInfoEXT {
    type Type = DebugMarkerMarkerInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugMarkerMarkerInfoEXTBuilder<'a> {
    inner: vk::DebugMarkerMarkerInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DebugMarkerMarkerInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_marker_name(mut self, p_marker_name: &'a CStr) -> Self {
        self.inner.p_marker_name = p_marker_name.as_ptr();
        self
    }
}
impl<'a> Deref for DebugMarkerMarkerInfoEXTBuilder<'a> {
    type Target = vk::DebugMarkerMarkerInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DedicatedAllocationImageCreateInfoNV {
    type Type = DedicatedAllocationImageCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DedicatedAllocationImageCreateInfoNVBuilder {
    inner: vk::DedicatedAllocationImageCreateInfoNV,
}
impl DedicatedAllocationImageCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dedicated_allocation(mut self, dedicated_allocation: bool) -> Self {
        self.inner.dedicated_allocation = if dedicated_allocation { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for DedicatedAllocationImageCreateInfoNVBuilder {
    type Target = vk::DedicatedAllocationImageCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageCreateInfoNext for DedicatedAllocationImageCreateInfoNVBuilder {}
impl ImageCreateInfoNext for vk::DedicatedAllocationImageCreateInfoNV {}
impl Builder<'_> for vk::DedicatedAllocationBufferCreateInfoNV {
    type Type = DedicatedAllocationBufferCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DedicatedAllocationBufferCreateInfoNVBuilder {
    inner: vk::DedicatedAllocationBufferCreateInfoNV,
}
impl DedicatedAllocationBufferCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dedicated_allocation(mut self, dedicated_allocation: bool) -> Self {
        self.inner.dedicated_allocation = if dedicated_allocation { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for DedicatedAllocationBufferCreateInfoNVBuilder {
    type Target = vk::DedicatedAllocationBufferCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl BufferCreateInfoNext for DedicatedAllocationBufferCreateInfoNVBuilder {}
impl BufferCreateInfoNext for vk::DedicatedAllocationBufferCreateInfoNV {}
impl Builder<'_> for vk::DedicatedAllocationMemoryAllocateInfoNV {
    type Type = DedicatedAllocationMemoryAllocateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DedicatedAllocationMemoryAllocateInfoNVBuilder {
    inner: vk::DedicatedAllocationMemoryAllocateInfoNV,
}
impl DedicatedAllocationMemoryAllocateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image(mut self, image: Option<vk::Image>) -> Self {
        self.inner.image = image;
        self
    }
    pub fn buffer(mut self, buffer: Option<vk::Buffer>) -> Self {
        self.inner.buffer = buffer;
        self
    }
}
impl Deref for DedicatedAllocationMemoryAllocateInfoNVBuilder {
    type Target = vk::DedicatedAllocationMemoryAllocateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for DedicatedAllocationMemoryAllocateInfoNVBuilder {}
impl MemoryAllocateInfoNext for vk::DedicatedAllocationMemoryAllocateInfoNV {}
impl Builder<'_> for vk::ExternalMemoryImageCreateInfoNV {
    type Type = ExternalMemoryImageCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExternalMemoryImageCreateInfoNVBuilder {
    inner: vk::ExternalMemoryImageCreateInfoNV,
}
impl ExternalMemoryImageCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalMemoryHandleTypeFlagsNV) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl Deref for ExternalMemoryImageCreateInfoNVBuilder {
    type Target = vk::ExternalMemoryImageCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageCreateInfoNext for ExternalMemoryImageCreateInfoNVBuilder {}
impl ImageCreateInfoNext for vk::ExternalMemoryImageCreateInfoNV {}
impl Builder<'_> for vk::ExportMemoryAllocateInfoNV {
    type Type = ExportMemoryAllocateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportMemoryAllocateInfoNVBuilder {
    inner: vk::ExportMemoryAllocateInfoNV,
}
impl ExportMemoryAllocateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalMemoryHandleTypeFlagsNV) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl Deref for ExportMemoryAllocateInfoNVBuilder {
    type Target = vk::ExportMemoryAllocateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for ExportMemoryAllocateInfoNVBuilder {}
impl MemoryAllocateInfoNext for vk::ExportMemoryAllocateInfoNV {}
impl Builder<'_> for vk::ImportMemoryWin32HandleInfoNV {
    type Type = ImportMemoryWin32HandleInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportMemoryWin32HandleInfoNVBuilder {
    inner: vk::ImportMemoryWin32HandleInfoNV,
}
impl ImportMemoryWin32HandleInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlagsNV) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn handle(mut self, handle: vk::HANDLE) -> Self {
        self.inner.handle = handle;
        self
    }
}
impl Deref for ImportMemoryWin32HandleInfoNVBuilder {
    type Target = vk::ImportMemoryWin32HandleInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for ImportMemoryWin32HandleInfoNVBuilder {}
impl MemoryAllocateInfoNext for vk::ImportMemoryWin32HandleInfoNV {}
impl<'a> Builder<'a> for vk::ExportMemoryWin32HandleInfoNV {
    type Type = ExportMemoryWin32HandleInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportMemoryWin32HandleInfoNVBuilder<'a> {
    inner: vk::ExportMemoryWin32HandleInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ExportMemoryWin32HandleInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attributes(mut self, p_attributes: Option<&'a vk::SECURITY_ATTRIBUTES>) -> Self {
        self.inner.p_attributes = p_attributes.map_or(ptr::null(), |p| p);
        self
    }
    pub fn dw_access(mut self, dw_access: vk::DWORD) -> Self {
        self.inner.dw_access = dw_access;
        self
    }
}
impl<'a> Deref for ExportMemoryWin32HandleInfoNVBuilder<'a> {
    type Target = vk::ExportMemoryWin32HandleInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> MemoryAllocateInfoNext for ExportMemoryWin32HandleInfoNVBuilder<'a> {}
impl MemoryAllocateInfoNext for vk::ExportMemoryWin32HandleInfoNV {}
impl<'a> Builder<'a> for vk::Win32KeyedMutexAcquireReleaseInfoNV {
    type Type = Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
    inner: vk::Win32KeyedMutexAcquireReleaseInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_acquire_syncs(
        mut self,
        p_acquire_syncs: &'a [vk::DeviceMemory],
        p_acquire_keys: &'a [u64],
        p_acquire_timeout_milliseconds: &'a [u32],
    ) -> Self {
        self.inner.acquire_count = p_acquire_syncs.len() as u32;
        assert_eq!(self.inner.acquire_count, p_acquire_keys.len() as u32);
        assert_eq!(self.inner.acquire_count, p_acquire_timeout_milliseconds.len() as u32);
        self.inner.p_acquire_syncs = p_acquire_syncs.first().map_or(ptr::null(), |s| s as *const _);
        self.inner.p_acquire_keys = p_acquire_keys.first().map_or(ptr::null(), |s| s as *const _);
        self.inner.p_acquire_timeout_milliseconds = p_acquire_timeout_milliseconds
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_release_syncs(mut self, p_release_syncs: &'a [vk::DeviceMemory], p_release_keys: &'a [u64]) -> Self {
        self.inner.release_count = p_release_syncs.len() as u32;
        assert_eq!(self.inner.release_count, p_release_keys.len() as u32);
        self.inner.p_release_syncs = p_release_syncs.first().map_or(ptr::null(), |s| s as *const _);
        self.inner.p_release_keys = p_release_keys.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
    type Target = vk::Win32KeyedMutexAcquireReleaseInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> SubmitInfoNext for Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {}
impl<'a> SubmitInfo2Next for Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {}
impl SubmitInfoNext for vk::Win32KeyedMutexAcquireReleaseInfoNV {}
impl SubmitInfo2Next for vk::Win32KeyedMutexAcquireReleaseInfoNV {}
impl Builder<'_> for vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    type Type = PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder {
    inner: vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV,
}
impl PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_generated_commands(mut self, device_generated_commands: bool) -> Self {
        self.inner.device_generated_commands = if device_generated_commands { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {}
impl Builder<'_> for vk::DevicePrivateDataCreateInfo {
    type Type = DevicePrivateDataCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DevicePrivateDataCreateInfoBuilder {
    inner: vk::DevicePrivateDataCreateInfo,
}
impl DevicePrivateDataCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn private_data_slot_request_count(mut self, private_data_slot_request_count: u32) -> Self {
        self.inner.private_data_slot_request_count = private_data_slot_request_count;
        self
    }
}
impl Deref for DevicePrivateDataCreateInfoBuilder {
    type Target = vk::DevicePrivateDataCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DeviceCreateInfoNext for DevicePrivateDataCreateInfoBuilder {}
impl DeviceCreateInfoNext for vk::DevicePrivateDataCreateInfo {}
impl Builder<'_> for vk::PrivateDataSlotCreateInfo {
    type Type = PrivateDataSlotCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PrivateDataSlotCreateInfoBuilder {
    inner: vk::PrivateDataSlotCreateInfo,
}
impl PrivateDataSlotCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PrivateDataSlotCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for PrivateDataSlotCreateInfoBuilder {
    type Target = vk::PrivateDataSlotCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDevicePrivateDataFeatures {
    type Type = PhysicalDevicePrivateDataFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePrivateDataFeaturesBuilder {
    inner: vk::PhysicalDevicePrivateDataFeatures,
}
impl PhysicalDevicePrivateDataFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn private_data(mut self, private_data: bool) -> Self {
        self.inner.private_data = if private_data { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDevicePrivateDataFeaturesBuilder {
    type Target = vk::PhysicalDevicePrivateDataFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDevicePrivateDataFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDevicePrivateDataFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePrivateDataFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePrivateDataFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMultiDrawPropertiesEXT {}
impl<'a> Builder<'a> for vk::GraphicsShaderGroupCreateInfoNV {
    type Type = GraphicsShaderGroupCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    inner: vk::GraphicsShaderGroupCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_stages(mut self, p_stages: &'a [vk::PipelineShaderStageCreateInfo]) -> Self {
        self.inner.stage_count = p_stages.len() as u32;
        self.inner.p_stages = p_stages.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_vertex_input_state(
        mut self,
        p_vertex_input_state: Option<&'a vk::PipelineVertexInputStateCreateInfo>,
    ) -> Self {
        self.inner.p_vertex_input_state = p_vertex_input_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_tessellation_state(
        mut self,
        p_tessellation_state: Option<&'a vk::PipelineTessellationStateCreateInfo>,
    ) -> Self {
        self.inner.p_tessellation_state = p_tessellation_state.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    type Target = vk::GraphicsShaderGroupCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::GraphicsPipelineShaderGroupsCreateInfoNV {
    type Type = GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
    inner: vk::GraphicsPipelineShaderGroupsCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_groups(mut self, p_groups: &'a [vk::GraphicsShaderGroupCreateInfoNV]) -> Self {
        self.inner.group_count = p_groups.len() as u32;
        self.inner.p_groups = p_groups.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_pipelines(mut self, p_pipelines: &'a [vk::Pipeline]) -> Self {
        self.inner.pipeline_count = p_pipelines.len() as u32;
        self.inner.p_pipelines = p_pipelines.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
    type Target = vk::GraphicsPipelineShaderGroupsCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> GraphicsPipelineCreateInfoNext for GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {}
impl GraphicsPipelineCreateInfoNext for vk::GraphicsPipelineShaderGroupsCreateInfoNV {}
impl<'a> Builder<'a> for vk::IndirectCommandsLayoutTokenNV {
    type Type = IndirectCommandsLayoutTokenNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct IndirectCommandsLayoutTokenNVBuilder<'a> {
    inner: vk::IndirectCommandsLayoutTokenNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> IndirectCommandsLayoutTokenNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn token_type(mut self, token_type: vk::IndirectCommandsTokenTypeNV) -> Self {
        self.inner.token_type = token_type;
        self
    }
    pub fn stream(mut self, stream: u32) -> Self {
        self.inner.stream = stream;
        self
    }
    pub fn offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn vertex_binding_unit(mut self, vertex_binding_unit: u32) -> Self {
        self.inner.vertex_binding_unit = vertex_binding_unit;
        self
    }
    pub fn vertex_dynamic_stride(mut self, vertex_dynamic_stride: bool) -> Self {
        self.inner.vertex_dynamic_stride = if vertex_dynamic_stride { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn pushconstant_pipeline_layout(mut self, pushconstant_pipeline_layout: Option<vk::PipelineLayout>) -> Self {
        self.inner.pushconstant_pipeline_layout = pushconstant_pipeline_layout;
        self
    }
    pub fn pushconstant_shader_stage_flags(mut self, pushconstant_shader_stage_flags: vk::ShaderStageFlags) -> Self {
        self.inner.pushconstant_shader_stage_flags = pushconstant_shader_stage_flags;
        self
    }
    pub fn pushconstant_offset(mut self, pushconstant_offset: u32) -> Self {
        self.inner.pushconstant_offset = pushconstant_offset;
        self
    }
    pub fn pushconstant_size(mut self, pushconstant_size: u32) -> Self {
        self.inner.pushconstant_size = pushconstant_size;
        self
    }
    pub fn indirect_state_flags(mut self, indirect_state_flags: vk::IndirectStateFlagsNV) -> Self {
        self.inner.indirect_state_flags = indirect_state_flags;
        self
    }
    pub fn p_index_types(mut self, p_index_types: &'a [vk::IndexType], p_index_type_values: &'a [u32]) -> Self {
        self.inner.index_type_count = p_index_types.len() as u32;
        assert_eq!(self.inner.index_type_count, p_index_type_values.len() as u32);
        self.inner.p_index_types = p_index_types.first().map_or(ptr::null(), |s| s as *const _);
        self.inner.p_index_type_values = p_index_type_values.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for IndirectCommandsLayoutTokenNVBuilder<'a> {
    type Target = vk::IndirectCommandsLayoutTokenNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::IndirectCommandsLayoutCreateInfoNV {
    type Type = IndirectCommandsLayoutCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
    inner: vk::IndirectCommandsLayoutCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::IndirectCommandsLayoutUsageFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn p_tokens(mut self, p_tokens: &'a [vk::IndirectCommandsLayoutTokenNV]) -> Self {
        self.inner.token_count = p_tokens.len() as u32;
        self.inner.p_tokens = p_tokens.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_stream_strides(mut self, p_stream_strides: &'a [u32]) -> Self {
        self.inner.stream_count = p_stream_strides.len() as u32;
        self.inner.p_stream_strides = p_stream_strides.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
    type Target = vk::IndirectCommandsLayoutCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::GeneratedCommandsInfoNV {
    type Type = GeneratedCommandsInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GeneratedCommandsInfoNVBuilder<'a> {
    inner: vk::GeneratedCommandsInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> GeneratedCommandsInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn pipeline(mut self, pipeline: vk::Pipeline) -> Self {
        self.inner.pipeline = Some(pipeline);
        self
    }
    pub fn indirect_commands_layout(mut self, indirect_commands_layout: vk::IndirectCommandsLayoutNV) -> Self {
        self.inner.indirect_commands_layout = Some(indirect_commands_layout);
        self
    }
    pub fn p_streams(mut self, p_streams: &'a [vk::IndirectCommandsStreamNV]) -> Self {
        self.inner.stream_count = p_streams.len() as u32;
        self.inner.p_streams = p_streams.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn sequences_count(mut self, sequences_count: u32) -> Self {
        self.inner.sequences_count = sequences_count;
        self
    }
    pub fn preprocess_buffer(mut self, preprocess_buffer: vk::Buffer) -> Self {
        self.inner.preprocess_buffer = Some(preprocess_buffer);
        self
    }
    pub fn preprocess_offset(mut self, preprocess_offset: vk::DeviceSize) -> Self {
        self.inner.preprocess_offset = preprocess_offset;
        self
    }
    pub fn preprocess_size(mut self, preprocess_size: vk::DeviceSize) -> Self {
        self.inner.preprocess_size = preprocess_size;
        self
    }
    pub fn sequences_count_buffer(mut self, sequences_count_buffer: Option<vk::Buffer>) -> Self {
        self.inner.sequences_count_buffer = sequences_count_buffer;
        self
    }
    pub fn sequences_count_offset(mut self, sequences_count_offset: vk::DeviceSize) -> Self {
        self.inner.sequences_count_offset = sequences_count_offset;
        self
    }
    pub fn sequences_index_buffer(mut self, sequences_index_buffer: Option<vk::Buffer>) -> Self {
        self.inner.sequences_index_buffer = sequences_index_buffer;
        self
    }
    pub fn sequences_index_offset(mut self, sequences_index_offset: vk::DeviceSize) -> Self {
        self.inner.sequences_index_offset = sequences_index_offset;
        self
    }
}
impl<'a> Deref for GeneratedCommandsInfoNVBuilder<'a> {
    type Target = vk::GeneratedCommandsInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::GeneratedCommandsMemoryRequirementsInfoNV {
    type Type = GeneratedCommandsMemoryRequirementsInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GeneratedCommandsMemoryRequirementsInfoNVBuilder {
    inner: vk::GeneratedCommandsMemoryRequirementsInfoNV,
}
impl GeneratedCommandsMemoryRequirementsInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn pipeline(mut self, pipeline: vk::Pipeline) -> Self {
        self.inner.pipeline = Some(pipeline);
        self
    }
    pub fn indirect_commands_layout(mut self, indirect_commands_layout: vk::IndirectCommandsLayoutNV) -> Self {
        self.inner.indirect_commands_layout = Some(indirect_commands_layout);
        self
    }
    pub fn max_sequences_count(mut self, max_sequences_count: u32) -> Self {
        self.inner.max_sequences_count = max_sequences_count;
        self
    }
}
impl Deref for GeneratedCommandsMemoryRequirementsInfoNVBuilder {
    type Target = vk::GeneratedCommandsMemoryRequirementsInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceFeatures2 {
    type Type = PhysicalDeviceFeatures2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PhysicalDeviceFeatures2Next {}
#[derive(Default)]
pub struct PhysicalDeviceFeatures2Builder<'a> {
    inner: vk::PhysicalDeviceFeatures2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PhysicalDeviceFeatures2Builder<'a> {
    pub fn insert_next<T: PhysicalDeviceFeatures2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn get_mut(&mut self) -> &mut vk::PhysicalDeviceFeatures2 {
        &mut self.inner
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn features(mut self, features: vk::PhysicalDeviceFeatures) -> Self {
        self.inner.features = features;
        self
    }
}
impl<'a> Deref for PhysicalDeviceFeatures2Builder<'a> {
    type Target = vk::PhysicalDeviceFeatures2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> DeviceCreateInfoNext for PhysicalDeviceFeatures2Builder<'a> {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFeatures2 {}
impl<'a> Builder<'a> for vk::PhysicalDeviceProperties2 {
    type Type = PhysicalDeviceProperties2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PhysicalDeviceProperties2Next {}
#[derive(Default)]
pub struct PhysicalDeviceProperties2Builder<'a> {
    inner: vk::PhysicalDeviceProperties2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PhysicalDeviceProperties2Builder<'a> {
    pub fn insert_next<T: PhysicalDeviceProperties2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn get_mut(&mut self) -> &mut vk::PhysicalDeviceProperties2 {
        &mut self.inner
    }
}
impl<'a> Deref for PhysicalDeviceProperties2Builder<'a> {
    type Target = vk::PhysicalDeviceProperties2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::FormatProperties2 {
    type Type = FormatProperties2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait FormatProperties2Next {}
#[derive(Default)]
pub struct FormatProperties2Builder<'a> {
    inner: vk::FormatProperties2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> FormatProperties2Builder<'a> {
    pub fn insert_next<T: FormatProperties2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn get_mut(&mut self) -> &mut vk::FormatProperties2 {
        &mut self.inner
    }
}
impl<'a> Deref for FormatProperties2Builder<'a> {
    type Target = vk::FormatProperties2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageFormatProperties2 {
    type Type = ImageFormatProperties2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ImageFormatProperties2Next {}
#[derive(Default)]
pub struct ImageFormatProperties2Builder<'a> {
    inner: vk::ImageFormatProperties2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ImageFormatProperties2Builder<'a> {
    pub fn insert_next<T: ImageFormatProperties2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn get_mut(&mut self) -> &mut vk::ImageFormatProperties2 {
        &mut self.inner
    }
}
impl<'a> Deref for ImageFormatProperties2Builder<'a> {
    type Target = vk::ImageFormatProperties2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceImageFormatInfo2 {
    type Type = PhysicalDeviceImageFormatInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PhysicalDeviceImageFormatInfo2Next {}
#[derive(Default)]
pub struct PhysicalDeviceImageFormatInfo2Builder<'a> {
    inner: vk::PhysicalDeviceImageFormatInfo2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PhysicalDeviceImageFormatInfo2Builder<'a> {
    pub fn insert_next<T: PhysicalDeviceImageFormatInfo2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn ty(mut self, ty: vk::ImageType) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn tiling(mut self, tiling: vk::ImageTiling) -> Self {
        self.inner.tiling = tiling;
        self
    }
    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn flags(mut self, flags: vk::ImageCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl<'a> Deref for PhysicalDeviceImageFormatInfo2Builder<'a> {
    type Target = vk::PhysicalDeviceImageFormatInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::QueueFamilyProperties2 {
    type Type = QueueFamilyProperties2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait QueueFamilyProperties2Next {}
#[derive(Default)]
pub struct QueueFamilyProperties2Builder<'a> {
    inner: vk::QueueFamilyProperties2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> QueueFamilyProperties2Builder<'a> {
    pub fn insert_next<T: QueueFamilyProperties2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn get_mut(&mut self) -> &mut vk::QueueFamilyProperties2 {
        &mut self.inner
    }
}
impl<'a> Deref for QueueFamilyProperties2Builder<'a> {
    type Target = vk::QueueFamilyProperties2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceMemoryProperties2 {
    type Type = PhysicalDeviceMemoryProperties2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PhysicalDeviceMemoryProperties2Next {}
#[derive(Default)]
pub struct PhysicalDeviceMemoryProperties2Builder<'a> {
    inner: vk::PhysicalDeviceMemoryProperties2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PhysicalDeviceMemoryProperties2Builder<'a> {
    pub fn insert_next<T: PhysicalDeviceMemoryProperties2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn get_mut(&mut self) -> &mut vk::PhysicalDeviceMemoryProperties2 {
        &mut self.inner
    }
}
impl<'a> Deref for PhysicalDeviceMemoryProperties2Builder<'a> {
    type Target = vk::PhysicalDeviceMemoryProperties2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceSparseImageFormatInfo2 {
    type Type = PhysicalDeviceSparseImageFormatInfo2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceSparseImageFormatInfo2Builder {
    inner: vk::PhysicalDeviceSparseImageFormatInfo2,
}
impl PhysicalDeviceSparseImageFormatInfo2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn ty(mut self, ty: vk::ImageType) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn samples(mut self, samples: vk::SampleCountFlags) -> Self {
        self.inner.samples = samples;
        self
    }
    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn tiling(mut self, tiling: vk::ImageTiling) -> Self {
        self.inner.tiling = tiling;
        self
    }
}
impl Deref for PhysicalDeviceSparseImageFormatInfo2Builder {
    type Target = vk::PhysicalDeviceSparseImageFormatInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceProperties2Next for vk::PhysicalDevicePushDescriptorPropertiesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDriverProperties {}
impl<'a> Builder<'a> for vk::PresentRegionsKHR {
    type Type = PresentRegionsKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PresentRegionsKHRBuilder<'a> {
    inner: vk::PresentRegionsKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PresentRegionsKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::PresentRegionKHR]) -> Self {
        self.inner.swapchain_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PresentRegionsKHRBuilder<'a> {
    type Target = vk::PresentRegionsKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> PresentInfoKHRNext for PresentRegionsKHRBuilder<'a> {}
impl PresentInfoKHRNext for vk::PresentRegionsKHR {}
impl<'a> Builder<'a> for vk::PresentRegionKHR {
    type Type = PresentRegionKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PresentRegionKHRBuilder<'a> {
    inner: vk::PresentRegionKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PresentRegionKHRBuilder<'a> {
    pub fn p_rectangles(mut self, p_rectangles: &'a [vk::RectLayerKHR]) -> Self {
        self.inner.rectangle_count = p_rectangles.len() as u32;
        self.inner.p_rectangles = p_rectangles.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PresentRegionKHRBuilder<'a> {
    type Target = vk::PresentRegionKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceVariablePointersFeatures {
    type Type = PhysicalDeviceVariablePointersFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVariablePointersFeaturesBuilder {
    inner: vk::PhysicalDeviceVariablePointersFeatures,
}
impl PhysicalDeviceVariablePointersFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn variable_pointers_storage_buffer(mut self, variable_pointers_storage_buffer: bool) -> Self {
        self.inner.variable_pointers_storage_buffer = if variable_pointers_storage_buffer {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn variable_pointers(mut self, variable_pointers: bool) -> Self {
        self.inner.variable_pointers = if variable_pointers { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceVariablePointersFeaturesBuilder {
    type Target = vk::PhysicalDeviceVariablePointersFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceVariablePointersFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceVariablePointersFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceVariablePointersFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceVariablePointersFeatures {}
impl Builder<'_> for vk::PhysicalDeviceExternalImageFormatInfo {
    type Type = PhysicalDeviceExternalImageFormatInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExternalImageFormatInfoBuilder {
    inner: vk::PhysicalDeviceExternalImageFormatInfo,
}
impl PhysicalDeviceExternalImageFormatInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for PhysicalDeviceExternalImageFormatInfoBuilder {
    type Target = vk::PhysicalDeviceExternalImageFormatInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceImageFormatInfo2Next for PhysicalDeviceExternalImageFormatInfoBuilder {}
impl PhysicalDeviceImageFormatInfo2Next for vk::PhysicalDeviceExternalImageFormatInfo {}
impl ImageFormatProperties2Next for vk::ExternalImageFormatProperties {}
impl Builder<'_> for vk::PhysicalDeviceExternalBufferInfo {
    type Type = PhysicalDeviceExternalBufferInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExternalBufferInfoBuilder {
    inner: vk::PhysicalDeviceExternalBufferInfo,
}
impl PhysicalDeviceExternalBufferInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::BufferCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn usage(mut self, usage: vk::BufferUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for PhysicalDeviceExternalBufferInfoBuilder {
    type Target = vk::PhysicalDeviceExternalBufferInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceIDProperties {}
impl Builder<'_> for vk::ExternalMemoryImageCreateInfo {
    type Type = ExternalMemoryImageCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExternalMemoryImageCreateInfoBuilder {
    inner: vk::ExternalMemoryImageCreateInfo,
}
impl ExternalMemoryImageCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl Deref for ExternalMemoryImageCreateInfoBuilder {
    type Target = vk::ExternalMemoryImageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageCreateInfoNext for ExternalMemoryImageCreateInfoBuilder {}
impl ImageCreateInfoNext for vk::ExternalMemoryImageCreateInfo {}
impl Builder<'_> for vk::ExternalMemoryBufferCreateInfo {
    type Type = ExternalMemoryBufferCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExternalMemoryBufferCreateInfoBuilder {
    inner: vk::ExternalMemoryBufferCreateInfo,
}
impl ExternalMemoryBufferCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl Deref for ExternalMemoryBufferCreateInfoBuilder {
    type Target = vk::ExternalMemoryBufferCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl BufferCreateInfoNext for ExternalMemoryBufferCreateInfoBuilder {}
impl BufferCreateInfoNext for vk::ExternalMemoryBufferCreateInfo {}
impl Builder<'_> for vk::ExportMemoryAllocateInfo {
    type Type = ExportMemoryAllocateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportMemoryAllocateInfoBuilder {
    inner: vk::ExportMemoryAllocateInfo,
}
impl ExportMemoryAllocateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl Deref for ExportMemoryAllocateInfoBuilder {
    type Target = vk::ExportMemoryAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for ExportMemoryAllocateInfoBuilder {}
impl MemoryAllocateInfoNext for vk::ExportMemoryAllocateInfo {}
impl Builder<'_> for vk::ImportMemoryWin32HandleInfoKHR {
    type Type = ImportMemoryWin32HandleInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportMemoryWin32HandleInfoKHRBuilder {
    inner: vk::ImportMemoryWin32HandleInfoKHR,
}
impl ImportMemoryWin32HandleInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn handle(mut self, handle: vk::HANDLE) -> Self {
        self.inner.handle = handle;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl Deref for ImportMemoryWin32HandleInfoKHRBuilder {
    type Target = vk::ImportMemoryWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for ImportMemoryWin32HandleInfoKHRBuilder {}
impl MemoryAllocateInfoNext for vk::ImportMemoryWin32HandleInfoKHR {}
impl<'a> Builder<'a> for vk::ExportMemoryWin32HandleInfoKHR {
    type Type = ExportMemoryWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    inner: vk::ExportMemoryWin32HandleInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attributes(mut self, p_attributes: Option<&'a vk::SECURITY_ATTRIBUTES>) -> Self {
        self.inner.p_attributes = p_attributes.map_or(ptr::null(), |p| p);
        self
    }
    pub fn dw_access(mut self, dw_access: vk::DWORD) -> Self {
        self.inner.dw_access = dw_access;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl<'a> Deref for ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    type Target = vk::ExportMemoryWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> MemoryAllocateInfoNext for ExportMemoryWin32HandleInfoKHRBuilder<'a> {}
impl MemoryAllocateInfoNext for vk::ExportMemoryWin32HandleInfoKHR {}
impl Builder<'_> for vk::ImportMemoryZirconHandleInfoFUCHSIA {
    type Type = ImportMemoryZirconHandleInfoFUCHSIABuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportMemoryZirconHandleInfoFUCHSIABuilder {
    inner: vk::ImportMemoryZirconHandleInfoFUCHSIA,
}
impl ImportMemoryZirconHandleInfoFUCHSIABuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn handle(mut self, handle: vk::zx_handle_t) -> Self {
        self.inner.handle = handle;
        self
    }
}
impl Deref for ImportMemoryZirconHandleInfoFUCHSIABuilder {
    type Target = vk::ImportMemoryZirconHandleInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for ImportMemoryZirconHandleInfoFUCHSIABuilder {}
impl MemoryAllocateInfoNext for vk::ImportMemoryZirconHandleInfoFUCHSIA {}
impl Builder<'_> for vk::MemoryGetZirconHandleInfoFUCHSIA {
    type Type = MemoryGetZirconHandleInfoFUCHSIABuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryGetZirconHandleInfoFUCHSIABuilder {
    inner: vk::MemoryGetZirconHandleInfoFUCHSIA,
}
impl MemoryGetZirconHandleInfoFUCHSIABuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for MemoryGetZirconHandleInfoFUCHSIABuilder {
    type Target = vk::MemoryGetZirconHandleInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::MemoryGetWin32HandleInfoKHR {
    type Type = MemoryGetWin32HandleInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryGetWin32HandleInfoKHRBuilder {
    inner: vk::MemoryGetWin32HandleInfoKHR,
}
impl MemoryGetWin32HandleInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for MemoryGetWin32HandleInfoKHRBuilder {
    type Target = vk::MemoryGetWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImportMemoryFdInfoKHR {
    type Type = ImportMemoryFdInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportMemoryFdInfoKHRBuilder {
    inner: vk::ImportMemoryFdInfoKHR,
}
impl ImportMemoryFdInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn fd(mut self, fd: c_int) -> Self {
        self.inner.fd = fd;
        self
    }
}
impl Deref for ImportMemoryFdInfoKHRBuilder {
    type Target = vk::ImportMemoryFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for ImportMemoryFdInfoKHRBuilder {}
impl MemoryAllocateInfoNext for vk::ImportMemoryFdInfoKHR {}
impl Builder<'_> for vk::MemoryGetFdInfoKHR {
    type Type = MemoryGetFdInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryGetFdInfoKHRBuilder {
    inner: vk::MemoryGetFdInfoKHR,
}
impl MemoryGetFdInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for MemoryGetFdInfoKHRBuilder {
    type Target = vk::MemoryGetFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::Win32KeyedMutexAcquireReleaseInfoKHR {
    type Type = Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    inner: vk::Win32KeyedMutexAcquireReleaseInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_acquire_syncs(
        mut self,
        p_acquire_syncs: &'a [vk::DeviceMemory],
        p_acquire_keys: &'a [u64],
        p_acquire_timeouts: &'a [u32],
    ) -> Self {
        self.inner.acquire_count = p_acquire_syncs.len() as u32;
        assert_eq!(self.inner.acquire_count, p_acquire_keys.len() as u32);
        assert_eq!(self.inner.acquire_count, p_acquire_timeouts.len() as u32);
        self.inner.p_acquire_syncs = p_acquire_syncs.first().map_or(ptr::null(), |s| s as *const _);
        self.inner.p_acquire_keys = p_acquire_keys.first().map_or(ptr::null(), |s| s as *const _);
        self.inner.p_acquire_timeouts = p_acquire_timeouts.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_release_syncs(mut self, p_release_syncs: &'a [vk::DeviceMemory], p_release_keys: &'a [u64]) -> Self {
        self.inner.release_count = p_release_syncs.len() as u32;
        assert_eq!(self.inner.release_count, p_release_keys.len() as u32);
        self.inner.p_release_syncs = p_release_syncs.first().map_or(ptr::null(), |s| s as *const _);
        self.inner.p_release_keys = p_release_keys.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    type Target = vk::Win32KeyedMutexAcquireReleaseInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> SubmitInfoNext for Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {}
impl<'a> SubmitInfo2Next for Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {}
impl SubmitInfoNext for vk::Win32KeyedMutexAcquireReleaseInfoKHR {}
impl SubmitInfo2Next for vk::Win32KeyedMutexAcquireReleaseInfoKHR {}
impl<'a> Builder<'a> for vk::PhysicalDeviceExternalSemaphoreInfo {
    type Type = PhysicalDeviceExternalSemaphoreInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PhysicalDeviceExternalSemaphoreInfoNext {}
#[derive(Default)]
pub struct PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    inner: vk::PhysicalDeviceExternalSemaphoreInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    pub fn insert_next<T: PhysicalDeviceExternalSemaphoreInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl<'a> Deref for PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    type Target = vk::PhysicalDeviceExternalSemaphoreInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ExportSemaphoreCreateInfo {
    type Type = ExportSemaphoreCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportSemaphoreCreateInfoBuilder {
    inner: vk::ExportSemaphoreCreateInfo,
}
impl ExportSemaphoreCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl Deref for ExportSemaphoreCreateInfoBuilder {
    type Target = vk::ExportSemaphoreCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SemaphoreCreateInfoNext for ExportSemaphoreCreateInfoBuilder {}
impl SemaphoreCreateInfoNext for vk::ExportSemaphoreCreateInfo {}
impl Builder<'_> for vk::ImportSemaphoreWin32HandleInfoKHR {
    type Type = ImportSemaphoreWin32HandleInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportSemaphoreWin32HandleInfoKHRBuilder {
    inner: vk::ImportSemaphoreWin32HandleInfoKHR,
}
impl ImportSemaphoreWin32HandleInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn flags(mut self, flags: vk::SemaphoreImportFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn handle(mut self, handle: vk::HANDLE) -> Self {
        self.inner.handle = handle;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl Deref for ImportSemaphoreWin32HandleInfoKHRBuilder {
    type Target = vk::ImportSemaphoreWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExportSemaphoreWin32HandleInfoKHR {
    type Type = ExportSemaphoreWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    inner: vk::ExportSemaphoreWin32HandleInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attributes(mut self, p_attributes: Option<&'a vk::SECURITY_ATTRIBUTES>) -> Self {
        self.inner.p_attributes = p_attributes.map_or(ptr::null(), |p| p);
        self
    }
    pub fn dw_access(mut self, dw_access: vk::DWORD) -> Self {
        self.inner.dw_access = dw_access;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl<'a> Deref for ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    type Target = vk::ExportSemaphoreWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> SemaphoreCreateInfoNext for ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {}
impl SemaphoreCreateInfoNext for vk::ExportSemaphoreWin32HandleInfoKHR {}
impl<'a> Builder<'a> for vk::D3D12FenceSubmitInfoKHR {
    type Type = D3D12FenceSubmitInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct D3D12FenceSubmitInfoKHRBuilder<'a> {
    inner: vk::D3D12FenceSubmitInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> D3D12FenceSubmitInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphore_values(mut self, p_wait_semaphore_values: &'a [u64]) -> Self {
        self.inner.wait_semaphore_values_count = p_wait_semaphore_values.len() as u32;
        self.inner.p_wait_semaphore_values = p_wait_semaphore_values.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_signal_semaphore_values(mut self, p_signal_semaphore_values: &'a [u64]) -> Self {
        self.inner.signal_semaphore_values_count = p_signal_semaphore_values.len() as u32;
        self.inner.p_signal_semaphore_values = p_signal_semaphore_values.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for D3D12FenceSubmitInfoKHRBuilder<'a> {
    type Target = vk::D3D12FenceSubmitInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> SubmitInfoNext for D3D12FenceSubmitInfoKHRBuilder<'a> {}
impl SubmitInfoNext for vk::D3D12FenceSubmitInfoKHR {}
impl Builder<'_> for vk::SemaphoreGetWin32HandleInfoKHR {
    type Type = SemaphoreGetWin32HandleInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreGetWin32HandleInfoKHRBuilder {
    inner: vk::SemaphoreGetWin32HandleInfoKHR,
}
impl SemaphoreGetWin32HandleInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for SemaphoreGetWin32HandleInfoKHRBuilder {
    type Target = vk::SemaphoreGetWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImportSemaphoreFdInfoKHR {
    type Type = ImportSemaphoreFdInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportSemaphoreFdInfoKHRBuilder {
    inner: vk::ImportSemaphoreFdInfoKHR,
}
impl ImportSemaphoreFdInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn flags(mut self, flags: vk::SemaphoreImportFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn fd(mut self, fd: c_int) -> Self {
        self.inner.fd = fd;
        self
    }
}
impl Deref for ImportSemaphoreFdInfoKHRBuilder {
    type Target = vk::ImportSemaphoreFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SemaphoreGetFdInfoKHR {
    type Type = SemaphoreGetFdInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreGetFdInfoKHRBuilder {
    inner: vk::SemaphoreGetFdInfoKHR,
}
impl SemaphoreGetFdInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for SemaphoreGetFdInfoKHRBuilder {
    type Target = vk::SemaphoreGetFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImportSemaphoreZirconHandleInfoFUCHSIA {
    type Type = ImportSemaphoreZirconHandleInfoFUCHSIABuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportSemaphoreZirconHandleInfoFUCHSIABuilder {
    inner: vk::ImportSemaphoreZirconHandleInfoFUCHSIA,
}
impl ImportSemaphoreZirconHandleInfoFUCHSIABuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn flags(mut self, flags: vk::SemaphoreImportFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn zircon_handle(mut self, zircon_handle: vk::zx_handle_t) -> Self {
        self.inner.zircon_handle = zircon_handle;
        self
    }
}
impl Deref for ImportSemaphoreZirconHandleInfoFUCHSIABuilder {
    type Target = vk::ImportSemaphoreZirconHandleInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SemaphoreGetZirconHandleInfoFUCHSIA {
    type Type = SemaphoreGetZirconHandleInfoFUCHSIABuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreGetZirconHandleInfoFUCHSIABuilder {
    inner: vk::SemaphoreGetZirconHandleInfoFUCHSIA,
}
impl SemaphoreGetZirconHandleInfoFUCHSIABuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for SemaphoreGetZirconHandleInfoFUCHSIABuilder {
    type Target = vk::SemaphoreGetZirconHandleInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceExternalFenceInfo {
    type Type = PhysicalDeviceExternalFenceInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExternalFenceInfoBuilder {
    inner: vk::PhysicalDeviceExternalFenceInfo,
}
impl PhysicalDeviceExternalFenceInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for PhysicalDeviceExternalFenceInfoBuilder {
    type Target = vk::PhysicalDeviceExternalFenceInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ExportFenceCreateInfo {
    type Type = ExportFenceCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportFenceCreateInfoBuilder {
    inner: vk::ExportFenceCreateInfo,
}
impl ExportFenceCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl Deref for ExportFenceCreateInfoBuilder {
    type Target = vk::ExportFenceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl FenceCreateInfoNext for ExportFenceCreateInfoBuilder {}
impl FenceCreateInfoNext for vk::ExportFenceCreateInfo {}
impl Builder<'_> for vk::ImportFenceWin32HandleInfoKHR {
    type Type = ImportFenceWin32HandleInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportFenceWin32HandleInfoKHRBuilder {
    inner: vk::ImportFenceWin32HandleInfoKHR,
}
impl ImportFenceWin32HandleInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fence(mut self, fence: vk::Fence) -> Self {
        self.inner.fence = Some(fence);
        self
    }
    pub fn flags(mut self, flags: vk::FenceImportFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn handle(mut self, handle: vk::HANDLE) -> Self {
        self.inner.handle = handle;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl Deref for ImportFenceWin32HandleInfoKHRBuilder {
    type Target = vk::ImportFenceWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExportFenceWin32HandleInfoKHR {
    type Type = ExportFenceWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportFenceWin32HandleInfoKHRBuilder<'a> {
    inner: vk::ExportFenceWin32HandleInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ExportFenceWin32HandleInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attributes(mut self, p_attributes: Option<&'a vk::SECURITY_ATTRIBUTES>) -> Self {
        self.inner.p_attributes = p_attributes.map_or(ptr::null(), |p| p);
        self
    }
    pub fn dw_access(mut self, dw_access: vk::DWORD) -> Self {
        self.inner.dw_access = dw_access;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl<'a> Deref for ExportFenceWin32HandleInfoKHRBuilder<'a> {
    type Target = vk::ExportFenceWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> FenceCreateInfoNext for ExportFenceWin32HandleInfoKHRBuilder<'a> {}
impl FenceCreateInfoNext for vk::ExportFenceWin32HandleInfoKHR {}
impl Builder<'_> for vk::FenceGetWin32HandleInfoKHR {
    type Type = FenceGetWin32HandleInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FenceGetWin32HandleInfoKHRBuilder {
    inner: vk::FenceGetWin32HandleInfoKHR,
}
impl FenceGetWin32HandleInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fence(mut self, fence: vk::Fence) -> Self {
        self.inner.fence = Some(fence);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for FenceGetWin32HandleInfoKHRBuilder {
    type Target = vk::FenceGetWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImportFenceFdInfoKHR {
    type Type = ImportFenceFdInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportFenceFdInfoKHRBuilder {
    inner: vk::ImportFenceFdInfoKHR,
}
impl ImportFenceFdInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fence(mut self, fence: vk::Fence) -> Self {
        self.inner.fence = Some(fence);
        self
    }
    pub fn flags(mut self, flags: vk::FenceImportFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn fd(mut self, fd: c_int) -> Self {
        self.inner.fd = fd;
        self
    }
}
impl Deref for ImportFenceFdInfoKHRBuilder {
    type Target = vk::ImportFenceFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::FenceGetFdInfoKHR {
    type Type = FenceGetFdInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FenceGetFdInfoKHRBuilder {
    inner: vk::FenceGetFdInfoKHR,
}
impl FenceGetFdInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fence(mut self, fence: vk::Fence) -> Self {
        self.inner.fence = Some(fence);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for FenceGetFdInfoKHRBuilder {
    type Target = vk::FenceGetFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceMultiviewFeatures {
    type Type = PhysicalDeviceMultiviewFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceMultiviewFeaturesBuilder {
    inner: vk::PhysicalDeviceMultiviewFeatures,
}
impl PhysicalDeviceMultiviewFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn multiview(mut self, multiview: bool) -> Self {
        self.inner.multiview = if multiview { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multiview_geometry_shader(mut self, multiview_geometry_shader: bool) -> Self {
        self.inner.multiview_geometry_shader = if multiview_geometry_shader { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multiview_tessellation_shader(mut self, multiview_tessellation_shader: bool) -> Self {
        self.inner.multiview_tessellation_shader = if multiview_tessellation_shader {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceMultiviewFeaturesBuilder {
    type Target = vk::PhysicalDeviceMultiviewFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceMultiviewFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceMultiviewFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMultiviewFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMultiviewFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMultiviewProperties {}
impl<'a> Builder<'a> for vk::RenderPassMultiviewCreateInfo {
    type Type = RenderPassMultiviewCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassMultiviewCreateInfoBuilder<'a> {
    inner: vk::RenderPassMultiviewCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RenderPassMultiviewCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_view_masks(mut self, p_view_masks: &'a [u32]) -> Self {
        self.inner.subpass_count = p_view_masks.len() as u32;
        self.inner.p_view_masks = p_view_masks.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_view_offsets(mut self, p_view_offsets: &'a [i32]) -> Self {
        self.inner.dependency_count = p_view_offsets.len() as u32;
        self.inner.p_view_offsets = p_view_offsets.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_correlation_masks(mut self, p_correlation_masks: &'a [u32]) -> Self {
        self.inner.correlation_mask_count = p_correlation_masks.len() as u32;
        self.inner.p_correlation_masks = p_correlation_masks.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for RenderPassMultiviewCreateInfoBuilder<'a> {
    type Target = vk::RenderPassMultiviewCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> RenderPassCreateInfoNext for RenderPassMultiviewCreateInfoBuilder<'a> {}
impl RenderPassCreateInfoNext for vk::RenderPassMultiviewCreateInfo {}
impl Builder<'_> for vk::DisplayPowerInfoEXT {
    type Type = DisplayPowerInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplayPowerInfoEXTBuilder {
    inner: vk::DisplayPowerInfoEXT,
}
impl DisplayPowerInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn power_state(mut self, power_state: vk::DisplayPowerStateEXT) -> Self {
        self.inner.power_state = power_state;
        self
    }
}
impl Deref for DisplayPowerInfoEXTBuilder {
    type Target = vk::DisplayPowerInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DeviceEventInfoEXT {
    type Type = DeviceEventInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceEventInfoEXTBuilder {
    inner: vk::DeviceEventInfoEXT,
}
impl DeviceEventInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_event(mut self, device_event: vk::DeviceEventTypeEXT) -> Self {
        self.inner.device_event = device_event;
        self
    }
}
impl Deref for DeviceEventInfoEXTBuilder {
    type Target = vk::DeviceEventInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DisplayEventInfoEXT {
    type Type = DisplayEventInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplayEventInfoEXTBuilder {
    inner: vk::DisplayEventInfoEXT,
}
impl DisplayEventInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn display_event(mut self, display_event: vk::DisplayEventTypeEXT) -> Self {
        self.inner.display_event = display_event;
        self
    }
}
impl Deref for DisplayEventInfoEXTBuilder {
    type Target = vk::DisplayEventInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SwapchainCounterCreateInfoEXT {
    type Type = SwapchainCounterCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SwapchainCounterCreateInfoEXTBuilder {
    inner: vk::SwapchainCounterCreateInfoEXT,
}
impl SwapchainCounterCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn surface_counters(mut self, surface_counters: vk::SurfaceCounterFlagsEXT) -> Self {
        self.inner.surface_counters = surface_counters;
        self
    }
}
impl Deref for SwapchainCounterCreateInfoEXTBuilder {
    type Target = vk::SwapchainCounterCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SwapchainCreateInfoKHRNext for SwapchainCounterCreateInfoEXTBuilder {}
impl SwapchainCreateInfoKHRNext for vk::SwapchainCounterCreateInfoEXT {}
impl Builder<'_> for vk::MemoryAllocateFlagsInfo {
    type Type = MemoryAllocateFlagsInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryAllocateFlagsInfoBuilder {
    inner: vk::MemoryAllocateFlagsInfo,
}
impl MemoryAllocateFlagsInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::MemoryAllocateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.inner.device_mask = device_mask;
        self
    }
}
impl Deref for MemoryAllocateFlagsInfoBuilder {
    type Target = vk::MemoryAllocateFlagsInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for MemoryAllocateFlagsInfoBuilder {}
impl MemoryAllocateInfoNext for vk::MemoryAllocateFlagsInfo {}
impl<'a> Builder<'a> for vk::BindBufferMemoryInfo {
    type Type = BindBufferMemoryInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait BindBufferMemoryInfoNext {}
#[derive(Default)]
pub struct BindBufferMemoryInfoBuilder<'a> {
    inner: vk::BindBufferMemoryInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BindBufferMemoryInfoBuilder<'a> {
    pub fn insert_next<T: BindBufferMemoryInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn memory_offset(mut self, memory_offset: vk::DeviceSize) -> Self {
        self.inner.memory_offset = memory_offset;
        self
    }
}
impl<'a> Deref for BindBufferMemoryInfoBuilder<'a> {
    type Target = vk::BindBufferMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BindBufferMemoryDeviceGroupInfo {
    type Type = BindBufferMemoryDeviceGroupInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    inner: vk::BindBufferMemoryDeviceGroupInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_device_indices(mut self, p_device_indices: &'a [u32]) -> Self {
        self.inner.device_index_count = p_device_indices.len() as u32;
        self.inner.p_device_indices = p_device_indices.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    type Target = vk::BindBufferMemoryDeviceGroupInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> BindBufferMemoryInfoNext for BindBufferMemoryDeviceGroupInfoBuilder<'a> {}
impl BindBufferMemoryInfoNext for vk::BindBufferMemoryDeviceGroupInfo {}
impl<'a> Builder<'a> for vk::BindImageMemoryInfo {
    type Type = BindImageMemoryInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait BindImageMemoryInfoNext {}
#[derive(Default)]
pub struct BindImageMemoryInfoBuilder<'a> {
    inner: vk::BindImageMemoryInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BindImageMemoryInfoBuilder<'a> {
    pub fn insert_next<T: BindImageMemoryInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn memory_offset(mut self, memory_offset: vk::DeviceSize) -> Self {
        self.inner.memory_offset = memory_offset;
        self
    }
}
impl<'a> Deref for BindImageMemoryInfoBuilder<'a> {
    type Target = vk::BindImageMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BindImageMemoryDeviceGroupInfo {
    type Type = BindImageMemoryDeviceGroupInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindImageMemoryDeviceGroupInfoBuilder<'a> {
    inner: vk::BindImageMemoryDeviceGroupInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BindImageMemoryDeviceGroupInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_device_indices(mut self, p_device_indices: &'a [u32]) -> Self {
        self.inner.device_index_count = p_device_indices.len() as u32;
        self.inner.p_device_indices = p_device_indices.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_split_instance_bind_regions(mut self, p_split_instance_bind_regions: &'a [vk::Rect2D]) -> Self {
        self.inner.split_instance_bind_region_count = p_split_instance_bind_regions.len() as u32;
        self.inner.p_split_instance_bind_regions = p_split_instance_bind_regions
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for BindImageMemoryDeviceGroupInfoBuilder<'a> {
    type Target = vk::BindImageMemoryDeviceGroupInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> BindImageMemoryInfoNext for BindImageMemoryDeviceGroupInfoBuilder<'a> {}
impl BindImageMemoryInfoNext for vk::BindImageMemoryDeviceGroupInfo {}
impl<'a> Builder<'a> for vk::DeviceGroupRenderPassBeginInfo {
    type Type = DeviceGroupRenderPassBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupRenderPassBeginInfoBuilder<'a> {
    inner: vk::DeviceGroupRenderPassBeginInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DeviceGroupRenderPassBeginInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.inner.device_mask = device_mask;
        self
    }
    pub fn p_device_render_areas(mut self, p_device_render_areas: &'a [vk::Rect2D]) -> Self {
        self.inner.device_render_area_count = p_device_render_areas.len() as u32;
        self.inner.p_device_render_areas = p_device_render_areas.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for DeviceGroupRenderPassBeginInfoBuilder<'a> {
    type Target = vk::DeviceGroupRenderPassBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> RenderPassBeginInfoNext for DeviceGroupRenderPassBeginInfoBuilder<'a> {}
impl<'a> RenderingInfoNext for DeviceGroupRenderPassBeginInfoBuilder<'a> {}
impl RenderPassBeginInfoNext for vk::DeviceGroupRenderPassBeginInfo {}
impl RenderingInfoNext for vk::DeviceGroupRenderPassBeginInfo {}
impl Builder<'_> for vk::DeviceGroupCommandBufferBeginInfo {
    type Type = DeviceGroupCommandBufferBeginInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupCommandBufferBeginInfoBuilder {
    inner: vk::DeviceGroupCommandBufferBeginInfo,
}
impl DeviceGroupCommandBufferBeginInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.inner.device_mask = device_mask;
        self
    }
}
impl Deref for DeviceGroupCommandBufferBeginInfoBuilder {
    type Target = vk::DeviceGroupCommandBufferBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl CommandBufferBeginInfoNext for DeviceGroupCommandBufferBeginInfoBuilder {}
impl CommandBufferBeginInfoNext for vk::DeviceGroupCommandBufferBeginInfo {}
impl<'a> Builder<'a> for vk::DeviceGroupSubmitInfo {
    type Type = DeviceGroupSubmitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupSubmitInfoBuilder<'a> {
    inner: vk::DeviceGroupSubmitInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DeviceGroupSubmitInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphore_device_indices(mut self, p_wait_semaphore_device_indices: &'a [u32]) -> Self {
        self.inner.wait_semaphore_count = p_wait_semaphore_device_indices.len() as u32;
        self.inner.p_wait_semaphore_device_indices = p_wait_semaphore_device_indices
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_command_buffer_device_masks(mut self, p_command_buffer_device_masks: &'a [u32]) -> Self {
        self.inner.command_buffer_count = p_command_buffer_device_masks.len() as u32;
        self.inner.p_command_buffer_device_masks = p_command_buffer_device_masks
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_signal_semaphore_device_indices(mut self, p_signal_semaphore_device_indices: &'a [u32]) -> Self {
        self.inner.signal_semaphore_count = p_signal_semaphore_device_indices.len() as u32;
        self.inner.p_signal_semaphore_device_indices = p_signal_semaphore_device_indices
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for DeviceGroupSubmitInfoBuilder<'a> {
    type Target = vk::DeviceGroupSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> SubmitInfoNext for DeviceGroupSubmitInfoBuilder<'a> {}
impl SubmitInfoNext for vk::DeviceGroupSubmitInfo {}
impl Builder<'_> for vk::DeviceGroupBindSparseInfo {
    type Type = DeviceGroupBindSparseInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupBindSparseInfoBuilder {
    inner: vk::DeviceGroupBindSparseInfo,
}
impl DeviceGroupBindSparseInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn resource_device_index(mut self, resource_device_index: u32) -> Self {
        self.inner.resource_device_index = resource_device_index;
        self
    }
    pub fn memory_device_index(mut self, memory_device_index: u32) -> Self {
        self.inner.memory_device_index = memory_device_index;
        self
    }
}
impl Deref for DeviceGroupBindSparseInfoBuilder {
    type Target = vk::DeviceGroupBindSparseInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl BindSparseInfoNext for DeviceGroupBindSparseInfoBuilder {}
impl BindSparseInfoNext for vk::DeviceGroupBindSparseInfo {}
impl Builder<'_> for vk::ImageSwapchainCreateInfoKHR {
    type Type = ImageSwapchainCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageSwapchainCreateInfoKHRBuilder {
    inner: vk::ImageSwapchainCreateInfoKHR,
}
impl ImageSwapchainCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn swapchain(mut self, swapchain: Option<vk::SwapchainKHR>) -> Self {
        self.inner.swapchain = swapchain;
        self
    }
}
impl Deref for ImageSwapchainCreateInfoKHRBuilder {
    type Target = vk::ImageSwapchainCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageCreateInfoNext for ImageSwapchainCreateInfoKHRBuilder {}
impl ImageCreateInfoNext for vk::ImageSwapchainCreateInfoKHR {}
impl Builder<'_> for vk::BindImageMemorySwapchainInfoKHR {
    type Type = BindImageMemorySwapchainInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindImageMemorySwapchainInfoKHRBuilder {
    inner: vk::BindImageMemorySwapchainInfoKHR,
}
impl BindImageMemorySwapchainInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn swapchain(mut self, swapchain: vk::SwapchainKHR) -> Self {
        self.inner.swapchain = Some(swapchain);
        self
    }
    pub fn image_index(mut self, image_index: u32) -> Self {
        self.inner.image_index = image_index;
        self
    }
}
impl Deref for BindImageMemorySwapchainInfoKHRBuilder {
    type Target = vk::BindImageMemorySwapchainInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl BindImageMemoryInfoNext for BindImageMemorySwapchainInfoKHRBuilder {}
impl BindImageMemoryInfoNext for vk::BindImageMemorySwapchainInfoKHR {}
impl Builder<'_> for vk::AcquireNextImageInfoKHR {
    type Type = AcquireNextImageInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AcquireNextImageInfoKHRBuilder {
    inner: vk::AcquireNextImageInfoKHR,
}
impl AcquireNextImageInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn swapchain(mut self, swapchain: vk::SwapchainKHR) -> Self {
        self.inner.swapchain = Some(swapchain);
        self
    }
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.inner.timeout = timeout;
        self
    }
    pub fn semaphore(mut self, semaphore: Option<vk::Semaphore>) -> Self {
        self.inner.semaphore = semaphore;
        self
    }
    pub fn fence(mut self, fence: Option<vk::Fence>) -> Self {
        self.inner.fence = fence;
        self
    }
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.inner.device_mask = device_mask;
        self
    }
}
impl Deref for AcquireNextImageInfoKHRBuilder {
    type Target = vk::AcquireNextImageInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceGroupPresentInfoKHR {
    type Type = DeviceGroupPresentInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupPresentInfoKHRBuilder<'a> {
    inner: vk::DeviceGroupPresentInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DeviceGroupPresentInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_device_masks(mut self, p_device_masks: &'a [u32]) -> Self {
        self.inner.swapchain_count = p_device_masks.len() as u32;
        self.inner.p_device_masks = p_device_masks.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn mode(mut self, mode: vk::DeviceGroupPresentModeFlagsKHR) -> Self {
        self.inner.mode = mode;
        self
    }
}
impl<'a> Deref for DeviceGroupPresentInfoKHRBuilder<'a> {
    type Target = vk::DeviceGroupPresentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> PresentInfoKHRNext for DeviceGroupPresentInfoKHRBuilder<'a> {}
impl PresentInfoKHRNext for vk::DeviceGroupPresentInfoKHR {}
impl<'a> Builder<'a> for vk::DeviceGroupDeviceCreateInfo {
    type Type = DeviceGroupDeviceCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupDeviceCreateInfoBuilder<'a> {
    inner: vk::DeviceGroupDeviceCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DeviceGroupDeviceCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_physical_devices(mut self, p_physical_devices: &'a [vk::PhysicalDevice]) -> Self {
        self.inner.physical_device_count = p_physical_devices.len() as u32;
        self.inner.p_physical_devices = p_physical_devices.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for DeviceGroupDeviceCreateInfoBuilder<'a> {
    type Target = vk::DeviceGroupDeviceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> DeviceCreateInfoNext for DeviceGroupDeviceCreateInfoBuilder<'a> {}
impl DeviceCreateInfoNext for vk::DeviceGroupDeviceCreateInfo {}
impl Builder<'_> for vk::DeviceGroupSwapchainCreateInfoKHR {
    type Type = DeviceGroupSwapchainCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupSwapchainCreateInfoKHRBuilder {
    inner: vk::DeviceGroupSwapchainCreateInfoKHR,
}
impl DeviceGroupSwapchainCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn modes(mut self, modes: vk::DeviceGroupPresentModeFlagsKHR) -> Self {
        self.inner.modes = modes;
        self
    }
}
impl Deref for DeviceGroupSwapchainCreateInfoKHRBuilder {
    type Target = vk::DeviceGroupSwapchainCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SwapchainCreateInfoKHRNext for DeviceGroupSwapchainCreateInfoKHRBuilder {}
impl SwapchainCreateInfoKHRNext for vk::DeviceGroupSwapchainCreateInfoKHR {}
impl<'a> Builder<'a> for vk::DescriptorUpdateTemplateCreateInfo {
    type Type = DescriptorUpdateTemplateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    inner: vk::DescriptorUpdateTemplateCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DescriptorUpdateTemplateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_descriptor_update_entries(
        mut self,
        p_descriptor_update_entries: &'a [vk::DescriptorUpdateTemplateEntry],
    ) -> Self {
        self.inner.descriptor_update_entry_count = p_descriptor_update_entries.len() as u32;
        self.inner.p_descriptor_update_entries = p_descriptor_update_entries
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn template_type(mut self, template_type: vk::DescriptorUpdateTemplateType) -> Self {
        self.inner.template_type = template_type;
        self
    }
    pub fn descriptor_set_layout(mut self, descriptor_set_layout: vk::DescriptorSetLayout) -> Self {
        self.inner.descriptor_set_layout = Some(descriptor_set_layout);
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn pipeline_layout(mut self, pipeline_layout: vk::PipelineLayout) -> Self {
        self.inner.pipeline_layout = Some(pipeline_layout);
        self
    }
    pub fn set(mut self, set: u32) -> Self {
        self.inner.set = set;
        self
    }
}
impl<'a> Deref for DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    type Target = vk::DescriptorUpdateTemplateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDevicePresentIdFeaturesKHR {
    type Type = PhysicalDevicePresentIdFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePresentIdFeaturesKHRBuilder {
    inner: vk::PhysicalDevicePresentIdFeaturesKHR,
}
impl PhysicalDevicePresentIdFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn present_id(mut self, present_id: bool) -> Self {
        self.inner.present_id = if present_id { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDevicePresentIdFeaturesKHRBuilder {
    type Target = vk::PhysicalDevicePresentIdFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDevicePresentIdFeaturesKHRBuilder {}
impl DeviceCreateInfoNext for PhysicalDevicePresentIdFeaturesKHRBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePresentIdFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePresentIdFeaturesKHR {}
impl<'a> Builder<'a> for vk::PresentIdKHR {
    type Type = PresentIdKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PresentIdKHRBuilder<'a> {
    inner: vk::PresentIdKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PresentIdKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_present_ids(mut self, p_present_ids: &'a [u64]) -> Self {
        self.inner.swapchain_count = p_present_ids.len() as u32;
        self.inner.p_present_ids = p_present_ids.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PresentIdKHRBuilder<'a> {
    type Target = vk::PresentIdKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> PresentInfoKHRNext for PresentIdKHRBuilder<'a> {}
impl PresentInfoKHRNext for vk::PresentIdKHR {}
impl Builder<'_> for vk::PhysicalDevicePresentWaitFeaturesKHR {
    type Type = PhysicalDevicePresentWaitFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePresentWaitFeaturesKHRBuilder {
    inner: vk::PhysicalDevicePresentWaitFeaturesKHR,
}
impl PhysicalDevicePresentWaitFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn present_wait(mut self, present_wait: bool) -> Self {
        self.inner.present_wait = if present_wait { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDevicePresentWaitFeaturesKHRBuilder {
    type Target = vk::PhysicalDevicePresentWaitFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDevicePresentWaitFeaturesKHRBuilder {}
impl DeviceCreateInfoNext for PhysicalDevicePresentWaitFeaturesKHRBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePresentWaitFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePresentWaitFeaturesKHR {}
impl Builder<'_> for vk::HdrMetadataEXT {
    type Type = HdrMetadataEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct HdrMetadataEXTBuilder {
    inner: vk::HdrMetadataEXT,
}
impl HdrMetadataEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn display_primary_red(mut self, display_primary_red: vk::XYColorEXT) -> Self {
        self.inner.display_primary_red = display_primary_red;
        self
    }
    pub fn display_primary_green(mut self, display_primary_green: vk::XYColorEXT) -> Self {
        self.inner.display_primary_green = display_primary_green;
        self
    }
    pub fn display_primary_blue(mut self, display_primary_blue: vk::XYColorEXT) -> Self {
        self.inner.display_primary_blue = display_primary_blue;
        self
    }
    pub fn white_point(mut self, white_point: vk::XYColorEXT) -> Self {
        self.inner.white_point = white_point;
        self
    }
    pub fn max_luminance(mut self, max_luminance: f32) -> Self {
        self.inner.max_luminance = max_luminance;
        self
    }
    pub fn min_luminance(mut self, min_luminance: f32) -> Self {
        self.inner.min_luminance = min_luminance;
        self
    }
    pub fn max_content_light_level(mut self, max_content_light_level: f32) -> Self {
        self.inner.max_content_light_level = max_content_light_level;
        self
    }
    pub fn max_frame_average_light_level(mut self, max_frame_average_light_level: f32) -> Self {
        self.inner.max_frame_average_light_level = max_frame_average_light_level;
        self
    }
}
impl Deref for HdrMetadataEXTBuilder {
    type Target = vk::HdrMetadataEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SurfaceCapabilities2KHRNext for vk::DisplayNativeHdrSurfaceCapabilitiesAMD {}
impl Builder<'_> for vk::SwapchainDisplayNativeHdrCreateInfoAMD {
    type Type = SwapchainDisplayNativeHdrCreateInfoAMDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMDBuilder {
    inner: vk::SwapchainDisplayNativeHdrCreateInfoAMD,
}
impl SwapchainDisplayNativeHdrCreateInfoAMDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn local_dimming_enable(mut self, local_dimming_enable: bool) -> Self {
        self.inner.local_dimming_enable = if local_dimming_enable { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for SwapchainDisplayNativeHdrCreateInfoAMDBuilder {
    type Target = vk::SwapchainDisplayNativeHdrCreateInfoAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SwapchainCreateInfoKHRNext for SwapchainDisplayNativeHdrCreateInfoAMDBuilder {}
impl SwapchainCreateInfoKHRNext for vk::SwapchainDisplayNativeHdrCreateInfoAMD {}
impl<'a> Builder<'a> for vk::PresentTimesInfoGOOGLE {
    type Type = PresentTimesInfoGOOGLEBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PresentTimesInfoGOOGLEBuilder<'a> {
    inner: vk::PresentTimesInfoGOOGLE,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PresentTimesInfoGOOGLEBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_times(mut self, p_times: &'a [vk::PresentTimeGOOGLE]) -> Self {
        self.inner.swapchain_count = p_times.len() as u32;
        self.inner.p_times = p_times.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PresentTimesInfoGOOGLEBuilder<'a> {
    type Target = vk::PresentTimesInfoGOOGLE;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> PresentInfoKHRNext for PresentTimesInfoGOOGLEBuilder<'a> {}
impl PresentInfoKHRNext for vk::PresentTimesInfoGOOGLE {}
impl Builder<'_> for vk::IOSSurfaceCreateInfoMVK {
    type Type = IOSSurfaceCreateInfoMVKBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct IOSSurfaceCreateInfoMVKBuilder {
    inner: vk::IOSSurfaceCreateInfoMVK,
}
impl IOSSurfaceCreateInfoMVKBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::IOSSurfaceCreateFlagsMVK) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_view(mut self, p_view: *const c_void) -> Self {
        self.inner.p_view = p_view;
        self
    }
}
impl Deref for IOSSurfaceCreateInfoMVKBuilder {
    type Target = vk::IOSSurfaceCreateInfoMVK;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::MacOSSurfaceCreateInfoMVK {
    type Type = MacOSSurfaceCreateInfoMVKBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MacOSSurfaceCreateInfoMVKBuilder {
    inner: vk::MacOSSurfaceCreateInfoMVK,
}
impl MacOSSurfaceCreateInfoMVKBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::MacOSSurfaceCreateFlagsMVK) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_view(mut self, p_view: *const c_void) -> Self {
        self.inner.p_view = p_view;
        self
    }
}
impl Deref for MacOSSurfaceCreateInfoMVKBuilder {
    type Target = vk::MacOSSurfaceCreateInfoMVK;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::MetalSurfaceCreateInfoEXT {
    type Type = MetalSurfaceCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MetalSurfaceCreateInfoEXTBuilder<'a> {
    inner: vk::MetalSurfaceCreateInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> MetalSurfaceCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::MetalSurfaceCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_layer(mut self, p_layer: &'a vk::CAMetalLayer) -> Self {
        self.inner.p_layer = p_layer;
        self
    }
}
impl<'a> Deref for MetalSurfaceCreateInfoEXTBuilder<'a> {
    type Target = vk::MetalSurfaceCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineViewportWScalingStateCreateInfoNV {
    type Type = PipelineViewportWScalingStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportWScalingStateCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn viewport_w_scaling_enable(mut self, viewport_w_scaling_enable: bool) -> Self {
        self.inner.viewport_w_scaling_enable = if viewport_w_scaling_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn viewport_count(mut self, viewport_count: u32) -> Self {
        self.inner.viewport_count = viewport_count;
        self
    }
    pub fn p_viewport_w_scalings(mut self, p_viewport_w_scalings: &'a [vk::ViewportWScalingNV]) -> Self {
        self.inner.viewport_count = p_viewport_w_scalings.len() as u32;
        self.inner.p_viewport_w_scalings = p_viewport_w_scalings.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineViewportWScalingStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> PipelineViewportStateCreateInfoNext for PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {}
impl PipelineViewportStateCreateInfoNext for vk::PipelineViewportWScalingStateCreateInfoNV {}
impl<'a> Builder<'a> for vk::PipelineViewportSwizzleStateCreateInfoNV {
    type Type = PipelineViewportSwizzleStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportSwizzleStateCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineViewportSwizzleStateCreateFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_viewport_swizzles(mut self, p_viewport_swizzles: &'a [vk::ViewportSwizzleNV]) -> Self {
        self.inner.viewport_count = p_viewport_swizzles.len() as u32;
        self.inner.p_viewport_swizzles = p_viewport_swizzles.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineViewportSwizzleStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> PipelineViewportStateCreateInfoNext for PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {}
impl PipelineViewportStateCreateInfoNext for vk::PipelineViewportSwizzleStateCreateInfoNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDiscardRectanglePropertiesEXT {}
impl<'a> Builder<'a> for vk::PipelineDiscardRectangleStateCreateInfoEXT {
    type Type = PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineDiscardRectangleStateCreateInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineDiscardRectangleStateCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn discard_rectangle_mode(mut self, discard_rectangle_mode: vk::DiscardRectangleModeEXT) -> Self {
        self.inner.discard_rectangle_mode = discard_rectangle_mode;
        self
    }
    pub fn p_discard_rectangles(mut self, p_discard_rectangles: &'a [vk::Rect2D]) -> Self {
        self.inner.discard_rectangle_count = p_discard_rectangles.len() as u32;
        self.inner.p_discard_rectangles = p_discard_rectangles.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineDiscardRectangleStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> GraphicsPipelineCreateInfoNext for PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {}
impl GraphicsPipelineCreateInfoNext for vk::PipelineDiscardRectangleStateCreateInfoEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {}
impl<'a> Builder<'a> for vk::RenderPassInputAttachmentAspectCreateInfo {
    type Type = RenderPassInputAttachmentAspectCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    inner: vk::RenderPassInputAttachmentAspectCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_aspect_references(mut self, p_aspect_references: &'a [vk::InputAttachmentAspectReference]) -> Self {
        self.inner.aspect_reference_count = p_aspect_references.len() as u32;
        self.inner.p_aspect_references = p_aspect_references.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    type Target = vk::RenderPassInputAttachmentAspectCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> RenderPassCreateInfoNext for RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {}
impl RenderPassCreateInfoNext for vk::RenderPassInputAttachmentAspectCreateInfo {}
impl<'a> Builder<'a> for vk::PhysicalDeviceSurfaceInfo2KHR {
    type Type = PhysicalDeviceSurfaceInfo2KHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PhysicalDeviceSurfaceInfo2KHRNext {}
#[derive(Default)]
pub struct PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
    inner: vk::PhysicalDeviceSurfaceInfo2KHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
    pub fn insert_next<T: PhysicalDeviceSurfaceInfo2KHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn surface(mut self, surface: Option<vk::SurfaceKHR>) -> Self {
        self.inner.surface = surface;
        self
    }
}
impl<'a> Deref for PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
    type Target = vk::PhysicalDeviceSurfaceInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SurfaceCapabilities2KHR {
    type Type = SurfaceCapabilities2KHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SurfaceCapabilities2KHRNext {}
#[derive(Default)]
pub struct SurfaceCapabilities2KHRBuilder<'a> {
    inner: vk::SurfaceCapabilities2KHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SurfaceCapabilities2KHRBuilder<'a> {
    pub fn insert_next<T: SurfaceCapabilities2KHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn get_mut(&mut self) -> &mut vk::SurfaceCapabilities2KHR {
        &mut self.inner
    }
}
impl<'a> Deref for SurfaceCapabilities2KHRBuilder<'a> {
    type Target = vk::SurfaceCapabilities2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DisplayPlaneInfo2KHR {
    type Type = DisplayPlaneInfo2KHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplayPlaneInfo2KHRBuilder {
    inner: vk::DisplayPlaneInfo2KHR,
}
impl DisplayPlaneInfo2KHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn mode(mut self, mode: vk::DisplayModeKHR) -> Self {
        self.inner.mode = Some(mode);
        self
    }
    pub fn plane_index(mut self, plane_index: u32) -> Self {
        self.inner.plane_index = plane_index;
        self
    }
}
impl Deref for DisplayPlaneInfo2KHRBuilder {
    type Target = vk::DisplayPlaneInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SurfaceCapabilities2KHRNext for vk::SharedPresentSurfaceCapabilitiesKHR {}
impl Builder<'_> for vk::PhysicalDevice16BitStorageFeatures {
    type Type = PhysicalDevice16BitStorageFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevice16BitStorageFeaturesBuilder {
    inner: vk::PhysicalDevice16BitStorageFeatures,
}
impl PhysicalDevice16BitStorageFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn storage_buffer16_bit_access(mut self, storage_buffer16_bit_access: bool) -> Self {
        self.inner.storage_buffer16_bit_access = if storage_buffer16_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn uniform_and_storage_buffer16_bit_access(mut self, uniform_and_storage_buffer16_bit_access: bool) -> Self {
        self.inner.uniform_and_storage_buffer16_bit_access = if uniform_and_storage_buffer16_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn storage_push_constant16(mut self, storage_push_constant16: bool) -> Self {
        self.inner.storage_push_constant16 = if storage_push_constant16 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn storage_input_output16(mut self, storage_input_output16: bool) -> Self {
        self.inner.storage_input_output16 = if storage_input_output16 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDevice16BitStorageFeaturesBuilder {
    type Target = vk::PhysicalDevice16BitStorageFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDevice16BitStorageFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDevice16BitStorageFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevice16BitStorageFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDevice16BitStorageFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceSubgroupProperties {}
impl Builder<'_> for vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    type Type = PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder {
    inner: vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures,
}
impl PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_subgroup_extended_types(mut self, shader_subgroup_extended_types: bool) -> Self {
        self.inner.shader_subgroup_extended_types = if shader_subgroup_extended_types {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder {
    type Target = vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures {}
impl Builder<'_> for vk::BufferMemoryRequirementsInfo2 {
    type Type = BufferMemoryRequirementsInfo2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferMemoryRequirementsInfo2Builder {
    inner: vk::BufferMemoryRequirementsInfo2,
}
impl BufferMemoryRequirementsInfo2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
}
impl Deref for BufferMemoryRequirementsInfo2Builder {
    type Target = vk::BufferMemoryRequirementsInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceBufferMemoryRequirements {
    type Type = DeviceBufferMemoryRequirementsBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceBufferMemoryRequirementsBuilder<'a> {
    inner: vk::DeviceBufferMemoryRequirements,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DeviceBufferMemoryRequirementsBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_create_info(mut self, p_create_info: &'a vk::BufferCreateInfo) -> Self {
        self.inner.p_create_info = p_create_info;
        self
    }
}
impl<'a> Deref for DeviceBufferMemoryRequirementsBuilder<'a> {
    type Target = vk::DeviceBufferMemoryRequirements;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageMemoryRequirementsInfo2 {
    type Type = ImageMemoryRequirementsInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ImageMemoryRequirementsInfo2Next {}
#[derive(Default)]
pub struct ImageMemoryRequirementsInfo2Builder<'a> {
    inner: vk::ImageMemoryRequirementsInfo2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ImageMemoryRequirementsInfo2Builder<'a> {
    pub fn insert_next<T: ImageMemoryRequirementsInfo2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
}
impl<'a> Deref for ImageMemoryRequirementsInfo2Builder<'a> {
    type Target = vk::ImageMemoryRequirementsInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImageSparseMemoryRequirementsInfo2 {
    type Type = ImageSparseMemoryRequirementsInfo2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageSparseMemoryRequirementsInfo2Builder {
    inner: vk::ImageSparseMemoryRequirementsInfo2,
}
impl ImageSparseMemoryRequirementsInfo2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
}
impl Deref for ImageSparseMemoryRequirementsInfo2Builder {
    type Target = vk::ImageSparseMemoryRequirementsInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceImageMemoryRequirements {
    type Type = DeviceImageMemoryRequirementsBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceImageMemoryRequirementsBuilder<'a> {
    inner: vk::DeviceImageMemoryRequirements,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DeviceImageMemoryRequirementsBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_create_info(mut self, p_create_info: &'a vk::ImageCreateInfo) -> Self {
        self.inner.p_create_info = p_create_info;
        self
    }
    pub fn plane_aspect(mut self, plane_aspect: vk::ImageAspectFlags) -> Self {
        self.inner.plane_aspect = plane_aspect;
        self
    }
}
impl<'a> Deref for DeviceImageMemoryRequirementsBuilder<'a> {
    type Target = vk::DeviceImageMemoryRequirements;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::MemoryRequirements2 {
    type Type = MemoryRequirements2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait MemoryRequirements2Next {}
#[derive(Default)]
pub struct MemoryRequirements2Builder<'a> {
    inner: vk::MemoryRequirements2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> MemoryRequirements2Builder<'a> {
    pub fn insert_next<T: MemoryRequirements2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn get_mut(&mut self) -> &mut vk::MemoryRequirements2 {
        &mut self.inner
    }
}
impl<'a> Deref for MemoryRequirements2Builder<'a> {
    type Target = vk::MemoryRequirements2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceProperties2Next for vk::PhysicalDevicePointClippingProperties {}
impl MemoryRequirements2Next for vk::MemoryDedicatedRequirements {}
impl Builder<'_> for vk::MemoryDedicatedAllocateInfo {
    type Type = MemoryDedicatedAllocateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryDedicatedAllocateInfoBuilder {
    inner: vk::MemoryDedicatedAllocateInfo,
}
impl MemoryDedicatedAllocateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image(mut self, image: Option<vk::Image>) -> Self {
        self.inner.image = image;
        self
    }
    pub fn buffer(mut self, buffer: Option<vk::Buffer>) -> Self {
        self.inner.buffer = buffer;
        self
    }
}
impl Deref for MemoryDedicatedAllocateInfoBuilder {
    type Target = vk::MemoryDedicatedAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for MemoryDedicatedAllocateInfoBuilder {}
impl MemoryAllocateInfoNext for vk::MemoryDedicatedAllocateInfo {}
impl Builder<'_> for vk::ImageViewUsageCreateInfo {
    type Type = ImageViewUsageCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageViewUsageCreateInfoBuilder {
    inner: vk::ImageViewUsageCreateInfo,
}
impl ImageViewUsageCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
}
impl Deref for ImageViewUsageCreateInfoBuilder {
    type Target = vk::ImageViewUsageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageViewCreateInfoNext for ImageViewUsageCreateInfoBuilder {}
impl ImageViewCreateInfoNext for vk::ImageViewUsageCreateInfo {}
impl Builder<'_> for vk::PipelineTessellationDomainOriginStateCreateInfo {
    type Type = PipelineTessellationDomainOriginStateCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineTessellationDomainOriginStateCreateInfoBuilder {
    inner: vk::PipelineTessellationDomainOriginStateCreateInfo,
}
impl PipelineTessellationDomainOriginStateCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn domain_origin(mut self, domain_origin: vk::TessellationDomainOrigin) -> Self {
        self.inner.domain_origin = domain_origin;
        self
    }
}
impl Deref for PipelineTessellationDomainOriginStateCreateInfoBuilder {
    type Target = vk::PipelineTessellationDomainOriginStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineTessellationStateCreateInfoNext for PipelineTessellationDomainOriginStateCreateInfoBuilder {}
impl PipelineTessellationStateCreateInfoNext for vk::PipelineTessellationDomainOriginStateCreateInfo {}
impl Builder<'_> for vk::SamplerYcbcrConversionInfo {
    type Type = SamplerYcbcrConversionInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SamplerYcbcrConversionInfoBuilder {
    inner: vk::SamplerYcbcrConversionInfo,
}
impl SamplerYcbcrConversionInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn conversion(mut self, conversion: vk::SamplerYcbcrConversion) -> Self {
        self.inner.conversion = Some(conversion);
        self
    }
}
impl Deref for SamplerYcbcrConversionInfoBuilder {
    type Target = vk::SamplerYcbcrConversionInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SamplerCreateInfoNext for SamplerYcbcrConversionInfoBuilder {}
impl ImageViewCreateInfoNext for SamplerYcbcrConversionInfoBuilder {}
impl SamplerCreateInfoNext for vk::SamplerYcbcrConversionInfo {}
impl ImageViewCreateInfoNext for vk::SamplerYcbcrConversionInfo {}
impl<'a> Builder<'a> for vk::SamplerYcbcrConversionCreateInfo {
    type Type = SamplerYcbcrConversionCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SamplerYcbcrConversionCreateInfoNext {}
#[derive(Default)]
pub struct SamplerYcbcrConversionCreateInfoBuilder<'a> {
    inner: vk::SamplerYcbcrConversionCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SamplerYcbcrConversionCreateInfoBuilder<'a> {
    pub fn insert_next<T: SamplerYcbcrConversionCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn ycbcr_model(mut self, ycbcr_model: vk::SamplerYcbcrModelConversion) -> Self {
        self.inner.ycbcr_model = ycbcr_model;
        self
    }
    pub fn ycbcr_range(mut self, ycbcr_range: vk::SamplerYcbcrRange) -> Self {
        self.inner.ycbcr_range = ycbcr_range;
        self
    }
    pub fn components(mut self, components: vk::ComponentMapping) -> Self {
        self.inner.components = components;
        self
    }
    pub fn x_chroma_offset(mut self, x_chroma_offset: vk::ChromaLocation) -> Self {
        self.inner.x_chroma_offset = x_chroma_offset;
        self
    }
    pub fn y_chroma_offset(mut self, y_chroma_offset: vk::ChromaLocation) -> Self {
        self.inner.y_chroma_offset = y_chroma_offset;
        self
    }
    pub fn chroma_filter(mut self, chroma_filter: vk::Filter) -> Self {
        self.inner.chroma_filter = chroma_filter;
        self
    }
    pub fn force_explicit_reconstruction(mut self, force_explicit_reconstruction: bool) -> Self {
        self.inner.force_explicit_reconstruction = if force_explicit_reconstruction {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for SamplerYcbcrConversionCreateInfoBuilder<'a> {
    type Target = vk::SamplerYcbcrConversionCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BindImagePlaneMemoryInfo {
    type Type = BindImagePlaneMemoryInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindImagePlaneMemoryInfoBuilder {
    inner: vk::BindImagePlaneMemoryInfo,
}
impl BindImagePlaneMemoryInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn plane_aspect(mut self, plane_aspect: vk::ImageAspectFlags) -> Self {
        self.inner.plane_aspect = plane_aspect;
        self
    }
}
impl Deref for BindImagePlaneMemoryInfoBuilder {
    type Target = vk::BindImagePlaneMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl BindImageMemoryInfoNext for BindImagePlaneMemoryInfoBuilder {}
impl BindImageMemoryInfoNext for vk::BindImagePlaneMemoryInfo {}
impl Builder<'_> for vk::ImagePlaneMemoryRequirementsInfo {
    type Type = ImagePlaneMemoryRequirementsInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImagePlaneMemoryRequirementsInfoBuilder {
    inner: vk::ImagePlaneMemoryRequirementsInfo,
}
impl ImagePlaneMemoryRequirementsInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn plane_aspect(mut self, plane_aspect: vk::ImageAspectFlags) -> Self {
        self.inner.plane_aspect = plane_aspect;
        self
    }
}
impl Deref for ImagePlaneMemoryRequirementsInfoBuilder {
    type Target = vk::ImagePlaneMemoryRequirementsInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageMemoryRequirementsInfo2Next for ImagePlaneMemoryRequirementsInfoBuilder {}
impl ImageMemoryRequirementsInfo2Next for vk::ImagePlaneMemoryRequirementsInfo {}
impl Builder<'_> for vk::PhysicalDeviceSamplerYcbcrConversionFeatures {
    type Type = PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder {
    inner: vk::PhysicalDeviceSamplerYcbcrConversionFeatures,
}
impl PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn sampler_ycbcr_conversion(mut self, sampler_ycbcr_conversion: bool) -> Self {
        self.inner.sampler_ycbcr_conversion = if sampler_ycbcr_conversion { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder {
    type Target = vk::PhysicalDeviceSamplerYcbcrConversionFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceSamplerYcbcrConversionFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceSamplerYcbcrConversionFeatures {}
impl ImageFormatProperties2Next for vk::SamplerYcbcrConversionImageFormatProperties {}
impl ImageFormatProperties2Next for vk::TextureLODGatherFormatPropertiesAMD {}
impl Builder<'_> for vk::ConditionalRenderingBeginInfoEXT {
    type Type = ConditionalRenderingBeginInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ConditionalRenderingBeginInfoEXTBuilder {
    inner: vk::ConditionalRenderingBeginInfoEXT,
}
impl ConditionalRenderingBeginInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn flags(mut self, flags: vk::ConditionalRenderingFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for ConditionalRenderingBeginInfoEXTBuilder {
    type Target = vk::ConditionalRenderingBeginInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ProtectedSubmitInfo {
    type Type = ProtectedSubmitInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ProtectedSubmitInfoBuilder {
    inner: vk::ProtectedSubmitInfo,
}
impl ProtectedSubmitInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn protected_submit(mut self, protected_submit: bool) -> Self {
        self.inner.protected_submit = if protected_submit { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for ProtectedSubmitInfoBuilder {
    type Target = vk::ProtectedSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubmitInfoNext for ProtectedSubmitInfoBuilder {}
impl SubmitInfoNext for vk::ProtectedSubmitInfo {}
impl Builder<'_> for vk::PhysicalDeviceProtectedMemoryFeatures {
    type Type = PhysicalDeviceProtectedMemoryFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceProtectedMemoryFeaturesBuilder {
    inner: vk::PhysicalDeviceProtectedMemoryFeatures,
}
impl PhysicalDeviceProtectedMemoryFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn protected_memory(mut self, protected_memory: bool) -> Self {
        self.inner.protected_memory = if protected_memory { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceProtectedMemoryFeaturesBuilder {
    type Target = vk::PhysicalDeviceProtectedMemoryFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceProtectedMemoryFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceProtectedMemoryFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceProtectedMemoryFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceProtectedMemoryFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceProtectedMemoryProperties {}
impl Builder<'_> for vk::DeviceQueueInfo2 {
    type Type = DeviceQueueInfo2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceQueueInfo2Builder {
    inner: vk::DeviceQueueInfo2,
}
impl DeviceQueueInfo2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DeviceQueueCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.inner.queue_family_index = queue_family_index;
        self
    }
    pub fn queue_index(mut self, queue_index: u32) -> Self {
        self.inner.queue_index = queue_index;
        self
    }
}
impl Deref for DeviceQueueInfo2Builder {
    type Target = vk::DeviceQueueInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineCoverageToColorStateCreateInfoNV {
    type Type = PipelineCoverageToColorStateCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCoverageToColorStateCreateInfoNVBuilder {
    inner: vk::PipelineCoverageToColorStateCreateInfoNV,
}
impl PipelineCoverageToColorStateCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCoverageToColorStateCreateFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn coverage_to_color_enable(mut self, coverage_to_color_enable: bool) -> Self {
        self.inner.coverage_to_color_enable = if coverage_to_color_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn coverage_to_color_location(mut self, coverage_to_color_location: u32) -> Self {
        self.inner.coverage_to_color_location = coverage_to_color_location;
        self
    }
}
impl Deref for PipelineCoverageToColorStateCreateInfoNVBuilder {
    type Target = vk::PipelineCoverageToColorStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineMultisampleStateCreateInfoNext for PipelineCoverageToColorStateCreateInfoNVBuilder {}
impl PipelineMultisampleStateCreateInfoNext for vk::PipelineCoverageToColorStateCreateInfoNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceSamplerFilterMinmaxProperties {}
impl<'a> Builder<'a> for vk::SampleLocationsInfoEXT {
    type Type = SampleLocationsInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SampleLocationsInfoEXTBuilder<'a> {
    inner: vk::SampleLocationsInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SampleLocationsInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn sample_locations_per_pixel(mut self, sample_locations_per_pixel: vk::SampleCountFlags) -> Self {
        self.inner.sample_locations_per_pixel = sample_locations_per_pixel;
        self
    }
    pub fn sample_location_grid_size(mut self, sample_location_grid_size: vk::Extent2D) -> Self {
        self.inner.sample_location_grid_size = sample_location_grid_size;
        self
    }
    pub fn p_sample_locations(mut self, p_sample_locations: &'a [vk::SampleLocationEXT]) -> Self {
        self.inner.sample_locations_count = p_sample_locations.len() as u32;
        self.inner.p_sample_locations = p_sample_locations.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for SampleLocationsInfoEXTBuilder<'a> {
    type Target = vk::SampleLocationsInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> ImageMemoryBarrierNext for SampleLocationsInfoEXTBuilder<'a> {}
impl<'a> ImageMemoryBarrier2Next for SampleLocationsInfoEXTBuilder<'a> {}
impl ImageMemoryBarrierNext for vk::SampleLocationsInfoEXT {}
impl ImageMemoryBarrier2Next for vk::SampleLocationsInfoEXT {}
impl<'a> Builder<'a> for vk::RenderPassSampleLocationsBeginInfoEXT {
    type Type = RenderPassSampleLocationsBeginInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    inner: vk::RenderPassSampleLocationsBeginInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attachment_initial_sample_locations(
        mut self,
        p_attachment_initial_sample_locations: &'a [vk::AttachmentSampleLocationsEXT],
    ) -> Self {
        self.inner.attachment_initial_sample_locations_count = p_attachment_initial_sample_locations.len() as u32;
        self.inner.p_attachment_initial_sample_locations = p_attachment_initial_sample_locations
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_post_subpass_sample_locations(
        mut self,
        p_post_subpass_sample_locations: &'a [vk::SubpassSampleLocationsEXT],
    ) -> Self {
        self.inner.post_subpass_sample_locations_count = p_post_subpass_sample_locations.len() as u32;
        self.inner.p_post_subpass_sample_locations = p_post_subpass_sample_locations
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    type Target = vk::RenderPassSampleLocationsBeginInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> RenderPassBeginInfoNext for RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {}
impl RenderPassBeginInfoNext for vk::RenderPassSampleLocationsBeginInfoEXT {}
impl Builder<'_> for vk::PipelineSampleLocationsStateCreateInfoEXT {
    type Type = PipelineSampleLocationsStateCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineSampleLocationsStateCreateInfoEXTBuilder {
    inner: vk::PipelineSampleLocationsStateCreateInfoEXT,
}
impl PipelineSampleLocationsStateCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn sample_locations_enable(mut self, sample_locations_enable: bool) -> Self {
        self.inner.sample_locations_enable = if sample_locations_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn sample_locations_info(mut self, sample_locations_info: vk::SampleLocationsInfoEXT) -> Self {
        self.inner.sample_locations_info = sample_locations_info;
        self
    }
}
impl Deref for PipelineSampleLocationsStateCreateInfoEXTBuilder {
    type Target = vk::PipelineSampleLocationsStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineMultisampleStateCreateInfoNext for PipelineSampleLocationsStateCreateInfoEXTBuilder {}
impl PipelineMultisampleStateCreateInfoNext for vk::PipelineSampleLocationsStateCreateInfoEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceSampleLocationsPropertiesEXT {}
impl Builder<'_> for vk::SamplerReductionModeCreateInfo {
    type Type = SamplerReductionModeCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SamplerReductionModeCreateInfoBuilder {
    inner: vk::SamplerReductionModeCreateInfo,
}
impl SamplerReductionModeCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn reduction_mode(mut self, reduction_mode: vk::SamplerReductionMode) -> Self {
        self.inner.reduction_mode = reduction_mode;
        self
    }
}
impl Deref for SamplerReductionModeCreateInfoBuilder {
    type Target = vk::SamplerReductionModeCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SamplerCreateInfoNext for SamplerReductionModeCreateInfoBuilder {}
impl SamplerCreateInfoNext for vk::SamplerReductionModeCreateInfo {}
impl Builder<'_> for vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    type Type = PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT,
}
impl PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn advanced_blend_coherent_operations(mut self, advanced_blend_coherent_operations: bool) -> Self {
        self.inner.advanced_blend_coherent_operations = if advanced_blend_coherent_operations {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT {}
impl Builder<'_> for vk::PhysicalDeviceMultiDrawFeaturesEXT {
    type Type = PhysicalDeviceMultiDrawFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceMultiDrawFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceMultiDrawFeaturesEXT,
}
impl PhysicalDeviceMultiDrawFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn multi_draw(mut self, multi_draw: bool) -> Self {
        self.inner.multi_draw = if multi_draw { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceMultiDrawFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceMultiDrawFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceMultiDrawFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceMultiDrawFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMultiDrawFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMultiDrawFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceBlendOperationAdvancedPropertiesEXT {}
impl Builder<'_> for vk::PipelineColorBlendAdvancedStateCreateInfoEXT {
    type Type = PipelineColorBlendAdvancedStateCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXTBuilder {
    inner: vk::PipelineColorBlendAdvancedStateCreateInfoEXT,
}
impl PipelineColorBlendAdvancedStateCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_premultiplied(mut self, src_premultiplied: bool) -> Self {
        self.inner.src_premultiplied = if src_premultiplied { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn dst_premultiplied(mut self, dst_premultiplied: bool) -> Self {
        self.inner.dst_premultiplied = if dst_premultiplied { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn blend_overlap(mut self, blend_overlap: vk::BlendOverlapEXT) -> Self {
        self.inner.blend_overlap = blend_overlap;
        self
    }
}
impl Deref for PipelineColorBlendAdvancedStateCreateInfoEXTBuilder {
    type Target = vk::PipelineColorBlendAdvancedStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineColorBlendStateCreateInfoNext for PipelineColorBlendAdvancedStateCreateInfoEXTBuilder {}
impl PipelineColorBlendStateCreateInfoNext for vk::PipelineColorBlendAdvancedStateCreateInfoEXT {}
impl Builder<'_> for vk::PhysicalDeviceInlineUniformBlockFeatures {
    type Type = PhysicalDeviceInlineUniformBlockFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceInlineUniformBlockFeaturesBuilder {
    inner: vk::PhysicalDeviceInlineUniformBlockFeatures,
}
impl PhysicalDeviceInlineUniformBlockFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn inline_uniform_block(mut self, inline_uniform_block: bool) -> Self {
        self.inner.inline_uniform_block = if inline_uniform_block { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn descriptor_binding_inline_uniform_block_update_after_bind(
        mut self,
        descriptor_binding_inline_uniform_block_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_inline_uniform_block_update_after_bind =
            if descriptor_binding_inline_uniform_block_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
}
impl Deref for PhysicalDeviceInlineUniformBlockFeaturesBuilder {
    type Target = vk::PhysicalDeviceInlineUniformBlockFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceInlineUniformBlockFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceInlineUniformBlockFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceInlineUniformBlockFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceInlineUniformBlockFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceInlineUniformBlockProperties {}
impl<'a> Builder<'a> for vk::WriteDescriptorSetInlineUniformBlock {
    type Type = WriteDescriptorSetInlineUniformBlockBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct WriteDescriptorSetInlineUniformBlockBuilder<'a> {
    inner: vk::WriteDescriptorSetInlineUniformBlock,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> WriteDescriptorSetInlineUniformBlockBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_data<T>(mut self, p_data: &'a [T]) -> Self {
        self.inner.data_size = mem::size_of_val(p_data) as u32;
        self.inner.p_data = p_data.first().map_or(ptr::null(), |s| s as *const _) as *const _;
        self
    }
}
impl<'a> Deref for WriteDescriptorSetInlineUniformBlockBuilder<'a> {
    type Target = vk::WriteDescriptorSetInlineUniformBlock;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> WriteDescriptorSetNext for WriteDescriptorSetInlineUniformBlockBuilder<'a> {}
impl WriteDescriptorSetNext for vk::WriteDescriptorSetInlineUniformBlock {}
impl Builder<'_> for vk::DescriptorPoolInlineUniformBlockCreateInfo {
    type Type = DescriptorPoolInlineUniformBlockCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorPoolInlineUniformBlockCreateInfoBuilder {
    inner: vk::DescriptorPoolInlineUniformBlockCreateInfo,
}
impl DescriptorPoolInlineUniformBlockCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn max_inline_uniform_block_bindings(mut self, max_inline_uniform_block_bindings: u32) -> Self {
        self.inner.max_inline_uniform_block_bindings = max_inline_uniform_block_bindings;
        self
    }
}
impl Deref for DescriptorPoolInlineUniformBlockCreateInfoBuilder {
    type Target = vk::DescriptorPoolInlineUniformBlockCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DescriptorPoolCreateInfoNext for DescriptorPoolInlineUniformBlockCreateInfoBuilder {}
impl DescriptorPoolCreateInfoNext for vk::DescriptorPoolInlineUniformBlockCreateInfo {}
impl<'a> Builder<'a> for vk::PipelineCoverageModulationStateCreateInfoNV {
    type Type = PipelineCoverageModulationStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineCoverageModulationStateCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCoverageModulationStateCreateFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn coverage_modulation_mode(mut self, coverage_modulation_mode: vk::CoverageModulationModeNV) -> Self {
        self.inner.coverage_modulation_mode = coverage_modulation_mode;
        self
    }
    pub fn coverage_modulation_table_enable(mut self, coverage_modulation_table_enable: bool) -> Self {
        self.inner.coverage_modulation_table_enable = if coverage_modulation_table_enable {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn coverage_modulation_table_count(mut self, coverage_modulation_table_count: u32) -> Self {
        self.inner.coverage_modulation_table_count = coverage_modulation_table_count;
        self
    }
    pub fn p_coverage_modulation_table(mut self, p_coverage_modulation_table: &'a [f32]) -> Self {
        self.inner.coverage_modulation_table_count = p_coverage_modulation_table.len() as u32;
        self.inner.p_coverage_modulation_table = p_coverage_modulation_table
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineCoverageModulationStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> PipelineMultisampleStateCreateInfoNext for PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {}
impl PipelineMultisampleStateCreateInfoNext for vk::PipelineCoverageModulationStateCreateInfoNV {}
impl<'a> Builder<'a> for vk::ImageFormatListCreateInfo {
    type Type = ImageFormatListCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageFormatListCreateInfoBuilder<'a> {
    inner: vk::ImageFormatListCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ImageFormatListCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_view_formats(mut self, p_view_formats: &'a [vk::Format]) -> Self {
        self.inner.view_format_count = p_view_formats.len() as u32;
        self.inner.p_view_formats = p_view_formats.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for ImageFormatListCreateInfoBuilder<'a> {
    type Target = vk::ImageFormatListCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> ImageCreateInfoNext for ImageFormatListCreateInfoBuilder<'a> {}
impl<'a> SwapchainCreateInfoKHRNext for ImageFormatListCreateInfoBuilder<'a> {}
impl<'a> PhysicalDeviceImageFormatInfo2Next for ImageFormatListCreateInfoBuilder<'a> {}
impl ImageCreateInfoNext for vk::ImageFormatListCreateInfo {}
impl SwapchainCreateInfoKHRNext for vk::ImageFormatListCreateInfo {}
impl PhysicalDeviceImageFormatInfo2Next for vk::ImageFormatListCreateInfo {}
impl<'a> Builder<'a> for vk::ValidationCacheCreateInfoEXT {
    type Type = ValidationCacheCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ValidationCacheCreateInfoEXTBuilder<'a> {
    inner: vk::ValidationCacheCreateInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ValidationCacheCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ValidationCacheCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_initial_data<T>(mut self, p_initial_data: &'a [T]) -> Self {
        self.inner.initial_data_size = mem::size_of_val(p_initial_data) as usize;
        self.inner.p_initial_data = p_initial_data.first().map_or(ptr::null(), |s| s as *const _) as *const _;
        self
    }
}
impl<'a> Deref for ValidationCacheCreateInfoEXTBuilder<'a> {
    type Target = vk::ValidationCacheCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ShaderModuleValidationCacheCreateInfoEXT {
    type Type = ShaderModuleValidationCacheCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ShaderModuleValidationCacheCreateInfoEXTBuilder {
    inner: vk::ShaderModuleValidationCacheCreateInfoEXT,
}
impl ShaderModuleValidationCacheCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn validation_cache(mut self, validation_cache: vk::ValidationCacheEXT) -> Self {
        self.inner.validation_cache = Some(validation_cache);
        self
    }
}
impl Deref for ShaderModuleValidationCacheCreateInfoEXTBuilder {
    type Target = vk::ShaderModuleValidationCacheCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ShaderModuleCreateInfoNext for ShaderModuleValidationCacheCreateInfoEXTBuilder {}
impl ShaderModuleCreateInfoNext for vk::ShaderModuleValidationCacheCreateInfoEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMaintenance3Properties {}
impl Builder<'_> for vk::PhysicalDeviceMaintenance4Features {
    type Type = PhysicalDeviceMaintenance4FeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceMaintenance4FeaturesBuilder {
    inner: vk::PhysicalDeviceMaintenance4Features,
}
impl PhysicalDeviceMaintenance4FeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn maintenance4(mut self, maintenance4: bool) -> Self {
        self.inner.maintenance4 = if maintenance4 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceMaintenance4FeaturesBuilder {
    type Target = vk::PhysicalDeviceMaintenance4Features;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceMaintenance4FeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceMaintenance4FeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMaintenance4Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMaintenance4Features {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMaintenance4Properties {}
impl<'a> Builder<'a> for vk::DescriptorSetLayoutSupport {
    type Type = DescriptorSetLayoutSupportBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DescriptorSetLayoutSupportNext {}
#[derive(Default)]
pub struct DescriptorSetLayoutSupportBuilder<'a> {
    inner: vk::DescriptorSetLayoutSupport,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DescriptorSetLayoutSupportBuilder<'a> {
    pub fn insert_next<T: DescriptorSetLayoutSupportNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn get_mut(&mut self) -> &mut vk::DescriptorSetLayoutSupport {
        &mut self.inner
    }
}
impl<'a> Deref for DescriptorSetLayoutSupportBuilder<'a> {
    type Target = vk::DescriptorSetLayoutSupport;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderDrawParametersFeatures {
    type Type = PhysicalDeviceShaderDrawParametersFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderDrawParametersFeaturesBuilder {
    inner: vk::PhysicalDeviceShaderDrawParametersFeatures,
}
impl PhysicalDeviceShaderDrawParametersFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_draw_parameters(mut self, shader_draw_parameters: bool) -> Self {
        self.inner.shader_draw_parameters = if shader_draw_parameters { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceShaderDrawParametersFeaturesBuilder {
    type Target = vk::PhysicalDeviceShaderDrawParametersFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceShaderDrawParametersFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceShaderDrawParametersFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderDrawParametersFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderDrawParametersFeatures {}
impl Builder<'_> for vk::PhysicalDeviceShaderFloat16Int8Features {
    type Type = PhysicalDeviceShaderFloat16Int8FeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderFloat16Int8FeaturesBuilder {
    inner: vk::PhysicalDeviceShaderFloat16Int8Features,
}
impl PhysicalDeviceShaderFloat16Int8FeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_float16(mut self, shader_float16: bool) -> Self {
        self.inner.shader_float16 = if shader_float16 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_int8(mut self, shader_int8: bool) -> Self {
        self.inner.shader_int8 = if shader_int8 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceShaderFloat16Int8FeaturesBuilder {
    type Target = vk::PhysicalDeviceShaderFloat16Int8Features;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceShaderFloat16Int8FeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceShaderFloat16Int8FeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderFloat16Int8Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderFloat16Int8Features {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceFloatControlsProperties {}
impl Builder<'_> for vk::PhysicalDeviceHostQueryResetFeatures {
    type Type = PhysicalDeviceHostQueryResetFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceHostQueryResetFeaturesBuilder {
    inner: vk::PhysicalDeviceHostQueryResetFeatures,
}
impl PhysicalDeviceHostQueryResetFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn host_query_reset(mut self, host_query_reset: bool) -> Self {
        self.inner.host_query_reset = if host_query_reset { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceHostQueryResetFeaturesBuilder {
    type Target = vk::PhysicalDeviceHostQueryResetFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceHostQueryResetFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceHostQueryResetFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceHostQueryResetFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceHostQueryResetFeatures {}
impl Builder<'_> for vk::DeviceQueueGlobalPriorityCreateInfoKHR {
    type Type = DeviceQueueGlobalPriorityCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceQueueGlobalPriorityCreateInfoKHRBuilder {
    inner: vk::DeviceQueueGlobalPriorityCreateInfoKHR,
}
impl DeviceQueueGlobalPriorityCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn global_priority(mut self, global_priority: vk::QueueGlobalPriorityKHR) -> Self {
        self.inner.global_priority = global_priority;
        self
    }
}
impl Deref for DeviceQueueGlobalPriorityCreateInfoKHRBuilder {
    type Target = vk::DeviceQueueGlobalPriorityCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DeviceQueueCreateInfoNext for DeviceQueueGlobalPriorityCreateInfoKHRBuilder {}
impl DeviceQueueCreateInfoNext for vk::DeviceQueueGlobalPriorityCreateInfoKHR {}
impl Builder<'_> for vk::PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    type Type = PhysicalDeviceGlobalPriorityQueryFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceGlobalPriorityQueryFeaturesKHRBuilder {
    inner: vk::PhysicalDeviceGlobalPriorityQueryFeaturesKHR,
}
impl PhysicalDeviceGlobalPriorityQueryFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn global_priority_query(mut self, global_priority_query: bool) -> Self {
        self.inner.global_priority_query = if global_priority_query { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceGlobalPriorityQueryFeaturesKHRBuilder {
    type Target = vk::PhysicalDeviceGlobalPriorityQueryFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceGlobalPriorityQueryFeaturesKHRBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceGlobalPriorityQueryFeaturesKHRBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceGlobalPriorityQueryFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceGlobalPriorityQueryFeaturesKHR {}
impl Builder<'_> for vk::QueueFamilyGlobalPriorityPropertiesKHR {
    type Type = QueueFamilyGlobalPriorityPropertiesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct QueueFamilyGlobalPriorityPropertiesKHRBuilder {
    inner: vk::QueueFamilyGlobalPriorityPropertiesKHR,
}
impl QueueFamilyGlobalPriorityPropertiesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn priority_count(mut self, priority_count: u32) -> Self {
        self.inner.priority_count = priority_count;
        self
    }
}
impl Deref for QueueFamilyGlobalPriorityPropertiesKHRBuilder {
    type Target = vk::QueueFamilyGlobalPriorityPropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl QueueFamilyProperties2Next for QueueFamilyGlobalPriorityPropertiesKHRBuilder {}
impl QueueFamilyProperties2Next for vk::QueueFamilyGlobalPriorityPropertiesKHR {}
impl<'a> Builder<'a> for vk::DebugUtilsObjectNameInfoEXT {
    type Type = DebugUtilsObjectNameInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugUtilsObjectNameInfoEXTBuilder<'a> {
    inner: vk::DebugUtilsObjectNameInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DebugUtilsObjectNameInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn object_type(mut self, object_type: vk::ObjectType) -> Self {
        self.inner.object_type = object_type;
        self
    }
    pub fn object_handle(mut self, object_handle: u64) -> Self {
        self.inner.object_handle = object_handle;
        self
    }
    pub fn p_object_name(mut self, p_object_name: Option<&'a CStr>) -> Self {
        self.inner.p_object_name = p_object_name.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
}
impl<'a> Deref for DebugUtilsObjectNameInfoEXTBuilder<'a> {
    type Target = vk::DebugUtilsObjectNameInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> PipelineShaderStageCreateInfoNext for DebugUtilsObjectNameInfoEXTBuilder<'a> {}
impl PipelineShaderStageCreateInfoNext for vk::DebugUtilsObjectNameInfoEXT {}
impl<'a> Builder<'a> for vk::DebugUtilsObjectTagInfoEXT {
    type Type = DebugUtilsObjectTagInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugUtilsObjectTagInfoEXTBuilder<'a> {
    inner: vk::DebugUtilsObjectTagInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DebugUtilsObjectTagInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn object_type(mut self, object_type: vk::ObjectType) -> Self {
        self.inner.object_type = object_type;
        self
    }
    pub fn object_handle(mut self, object_handle: u64) -> Self {
        self.inner.object_handle = object_handle;
        self
    }
    pub fn tag_name(mut self, tag_name: u64) -> Self {
        self.inner.tag_name = tag_name;
        self
    }
    pub fn p_tag<T>(mut self, p_tag: &'a [T]) -> Self {
        self.inner.tag_size = mem::size_of_val(p_tag) as usize;
        self.inner.p_tag = p_tag.first().map_or(ptr::null(), |s| s as *const _) as *const _;
        self
    }
}
impl<'a> Deref for DebugUtilsObjectTagInfoEXTBuilder<'a> {
    type Target = vk::DebugUtilsObjectTagInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DebugUtilsLabelEXT {
    type Type = DebugUtilsLabelEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugUtilsLabelEXTBuilder<'a> {
    inner: vk::DebugUtilsLabelEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DebugUtilsLabelEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_label_name(mut self, p_label_name: &'a CStr) -> Self {
        self.inner.p_label_name = p_label_name.as_ptr();
        self
    }
}
impl<'a> Deref for DebugUtilsLabelEXTBuilder<'a> {
    type Target = vk::DebugUtilsLabelEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DebugUtilsMessengerCreateInfoEXT {
    type Type = DebugUtilsMessengerCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugUtilsMessengerCreateInfoEXTBuilder {
    inner: vk::DebugUtilsMessengerCreateInfoEXT,
}
impl DebugUtilsMessengerCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DebugUtilsMessengerCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn message_severity(mut self, message_severity: vk::DebugUtilsMessageSeverityFlagsEXT) -> Self {
        self.inner.message_severity = message_severity;
        self
    }
    pub fn message_type(mut self, message_type: vk::DebugUtilsMessageTypeFlagsEXT) -> Self {
        self.inner.message_type = message_type;
        self
    }
    pub fn pfn_user_callback(mut self, pfn_user_callback: vk::FnDebugUtilsMessengerCallbackEXT) -> Self {
        self.inner.pfn_user_callback = Some(pfn_user_callback);
        self
    }
    pub fn p_user_data(mut self, p_user_data: *mut c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
}
impl Deref for DebugUtilsMessengerCreateInfoEXTBuilder {
    type Target = vk::DebugUtilsMessengerCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl InstanceCreateInfoNext for DebugUtilsMessengerCreateInfoEXTBuilder {}
impl InstanceCreateInfoNext for vk::DebugUtilsMessengerCreateInfoEXT {}
impl<'a> Builder<'a> for vk::DebugUtilsMessengerCallbackDataEXT {
    type Type = DebugUtilsMessengerCallbackDataEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    inner: vk::DebugUtilsMessengerCallbackDataEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DebugUtilsMessengerCallbackDataFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_message_id_name(mut self, p_message_id_name: Option<&'a CStr>) -> Self {
        self.inner.p_message_id_name = p_message_id_name.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn message_id_number(mut self, message_id_number: i32) -> Self {
        self.inner.message_id_number = message_id_number;
        self
    }
    pub fn p_message(mut self, p_message: &'a CStr) -> Self {
        self.inner.p_message = p_message.as_ptr();
        self
    }
    pub fn p_queue_labels(mut self, p_queue_labels: &'a [vk::DebugUtilsLabelEXT]) -> Self {
        self.inner.queue_label_count = p_queue_labels.len() as u32;
        self.inner.p_queue_labels = p_queue_labels.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_cmd_buf_labels(mut self, p_cmd_buf_labels: &'a [vk::DebugUtilsLabelEXT]) -> Self {
        self.inner.cmd_buf_label_count = p_cmd_buf_labels.len() as u32;
        self.inner.p_cmd_buf_labels = p_cmd_buf_labels.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_objects(mut self, p_objects: &'a [vk::DebugUtilsObjectNameInfoEXT]) -> Self {
        self.inner.object_count = p_objects.len() as u32;
        self.inner.p_objects = p_objects.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    type Target = vk::DebugUtilsMessengerCallbackDataEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    type Type = PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceDeviceMemoryReportFeaturesEXT,
}
impl PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_memory_report(mut self, device_memory_report: bool) -> Self {
        self.inner.device_memory_report = if device_memory_report { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceDeviceMemoryReportFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDeviceMemoryReportFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDeviceMemoryReportFeaturesEXT {}
impl Builder<'_> for vk::DeviceDeviceMemoryReportCreateInfoEXT {
    type Type = DeviceDeviceMemoryReportCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceDeviceMemoryReportCreateInfoEXTBuilder {
    inner: vk::DeviceDeviceMemoryReportCreateInfoEXT,
}
impl DeviceDeviceMemoryReportCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DeviceMemoryReportFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn pfn_user_callback(mut self, pfn_user_callback: vk::FnDeviceMemoryReportCallbackEXT) -> Self {
        self.inner.pfn_user_callback = Some(pfn_user_callback);
        self
    }
    pub fn p_user_data(mut self, p_user_data: *mut c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
}
impl Deref for DeviceDeviceMemoryReportCreateInfoEXTBuilder {
    type Target = vk::DeviceDeviceMemoryReportCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DeviceCreateInfoNext for DeviceDeviceMemoryReportCreateInfoEXTBuilder {}
impl DeviceCreateInfoNext for vk::DeviceDeviceMemoryReportCreateInfoEXT {}
impl Builder<'_> for vk::ImportMemoryHostPointerInfoEXT {
    type Type = ImportMemoryHostPointerInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportMemoryHostPointerInfoEXTBuilder {
    inner: vk::ImportMemoryHostPointerInfoEXT,
}
impl ImportMemoryHostPointerInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn p_host_pointer(mut self, p_host_pointer: *mut c_void) -> Self {
        self.inner.p_host_pointer = p_host_pointer;
        self
    }
}
impl Deref for ImportMemoryHostPointerInfoEXTBuilder {
    type Target = vk::ImportMemoryHostPointerInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for ImportMemoryHostPointerInfoEXTBuilder {}
impl MemoryAllocateInfoNext for vk::ImportMemoryHostPointerInfoEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceExternalMemoryHostPropertiesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceConservativeRasterizationPropertiesEXT {}
impl Builder<'_> for vk::CalibratedTimestampInfoEXT {
    type Type = CalibratedTimestampInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CalibratedTimestampInfoEXTBuilder {
    inner: vk::CalibratedTimestampInfoEXT,
}
impl CalibratedTimestampInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn time_domain(mut self, time_domain: vk::TimeDomainEXT) -> Self {
        self.inner.time_domain = time_domain;
        self
    }
}
impl Deref for CalibratedTimestampInfoEXTBuilder {
    type Target = vk::CalibratedTimestampInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceShaderCorePropertiesAMD {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceShaderCoreProperties2AMD {}
impl Builder<'_> for vk::PipelineRasterizationConservativeStateCreateInfoEXT {
    type Type = PipelineRasterizationConservativeStateCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationConservativeStateCreateInfoEXTBuilder {
    inner: vk::PipelineRasterizationConservativeStateCreateInfoEXT,
}
impl PipelineRasterizationConservativeStateCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineRasterizationConservativeStateCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn conservative_rasterization_mode(
        mut self,
        conservative_rasterization_mode: vk::ConservativeRasterizationModeEXT,
    ) -> Self {
        self.inner.conservative_rasterization_mode = conservative_rasterization_mode;
        self
    }
    pub fn extra_primitive_overestimation_size(mut self, extra_primitive_overestimation_size: f32) -> Self {
        self.inner.extra_primitive_overestimation_size = extra_primitive_overestimation_size;
        self
    }
}
impl Deref for PipelineRasterizationConservativeStateCreateInfoEXTBuilder {
    type Target = vk::PipelineRasterizationConservativeStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineRasterizationStateCreateInfoNext for PipelineRasterizationConservativeStateCreateInfoEXTBuilder {}
impl PipelineRasterizationStateCreateInfoNext for vk::PipelineRasterizationConservativeStateCreateInfoEXT {}
impl Builder<'_> for vk::PhysicalDeviceDescriptorIndexingFeatures {
    type Type = PhysicalDeviceDescriptorIndexingFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDescriptorIndexingFeaturesBuilder {
    inner: vk::PhysicalDeviceDescriptorIndexingFeatures,
}
impl PhysicalDeviceDescriptorIndexingFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_input_attachment_array_dynamic_indexing(
        mut self,
        shader_input_attachment_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_input_attachment_array_dynamic_indexing = if shader_input_attachment_array_dynamic_indexing {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_texel_buffer_array_dynamic_indexing =
            if shader_uniform_texel_buffer_array_dynamic_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_storage_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_texel_buffer_array_dynamic_indexing =
            if shader_storage_texel_buffer_array_dynamic_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_uniform_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_buffer_array_non_uniform_indexing =
            if shader_uniform_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_sampled_image_array_non_uniform_indexing(
        mut self,
        shader_sampled_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_sampled_image_array_non_uniform_indexing = if shader_sampled_image_array_non_uniform_indexing
        {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_storage_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_buffer_array_non_uniform_indexing =
            if shader_storage_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_image_array_non_uniform_indexing(
        mut self,
        shader_storage_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_image_array_non_uniform_indexing = if shader_storage_image_array_non_uniform_indexing
        {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_input_attachment_array_non_uniform_indexing(
        mut self,
        shader_input_attachment_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_input_attachment_array_non_uniform_indexing =
            if shader_input_attachment_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_texel_buffer_array_non_uniform_indexing =
            if shader_uniform_texel_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_texel_buffer_array_non_uniform_indexing =
            if shader_storage_texel_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_uniform_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_uniform_buffer_update_after_bind =
            if descriptor_binding_uniform_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_sampled_image_update_after_bind(
        mut self,
        descriptor_binding_sampled_image_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_sampled_image_update_after_bind =
            if descriptor_binding_sampled_image_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_image_update_after_bind(
        mut self,
        descriptor_binding_storage_image_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_image_update_after_bind =
            if descriptor_binding_storage_image_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_buffer_update_after_bind =
            if descriptor_binding_storage_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_uniform_texel_buffer_update_after_bind =
            if descriptor_binding_uniform_texel_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_texel_buffer_update_after_bind =
            if descriptor_binding_storage_texel_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_update_unused_while_pending(
        mut self,
        descriptor_binding_update_unused_while_pending: bool,
    ) -> Self {
        self.inner.descriptor_binding_update_unused_while_pending = if descriptor_binding_update_unused_while_pending {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn descriptor_binding_partially_bound(mut self, descriptor_binding_partially_bound: bool) -> Self {
        self.inner.descriptor_binding_partially_bound = if descriptor_binding_partially_bound {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn descriptor_binding_variable_descriptor_count(
        mut self,
        descriptor_binding_variable_descriptor_count: bool,
    ) -> Self {
        self.inner.descriptor_binding_variable_descriptor_count = if descriptor_binding_variable_descriptor_count {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn runtime_descriptor_array(mut self, runtime_descriptor_array: bool) -> Self {
        self.inner.runtime_descriptor_array = if runtime_descriptor_array { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceDescriptorIndexingFeaturesBuilder {
    type Target = vk::PhysicalDeviceDescriptorIndexingFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceDescriptorIndexingFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceDescriptorIndexingFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDescriptorIndexingFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDescriptorIndexingFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDescriptorIndexingProperties {}
impl<'a> Builder<'a> for vk::DescriptorSetLayoutBindingFlagsCreateInfo {
    type Type = DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
    inner: vk::DescriptorSetLayoutBindingFlagsCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_binding_flags(mut self, p_binding_flags: &'a [vk::DescriptorBindingFlags]) -> Self {
        self.inner.binding_count = p_binding_flags.len() as u32;
        self.inner.p_binding_flags = p_binding_flags.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
    type Target = vk::DescriptorSetLayoutBindingFlagsCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> DescriptorSetLayoutCreateInfoNext for DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {}
impl DescriptorSetLayoutCreateInfoNext for vk::DescriptorSetLayoutBindingFlagsCreateInfo {}
impl<'a> Builder<'a> for vk::DescriptorSetVariableDescriptorCountAllocateInfo {
    type Type = DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
    inner: vk::DescriptorSetVariableDescriptorCountAllocateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_descriptor_counts(mut self, p_descriptor_counts: &'a [u32]) -> Self {
        self.inner.descriptor_set_count = p_descriptor_counts.len() as u32;
        self.inner.p_descriptor_counts = p_descriptor_counts.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
    type Target = vk::DescriptorSetVariableDescriptorCountAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> DescriptorSetAllocateInfoNext for DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {}
impl DescriptorSetAllocateInfoNext for vk::DescriptorSetVariableDescriptorCountAllocateInfo {}
impl DescriptorSetLayoutSupportNext for vk::DescriptorSetVariableDescriptorCountLayoutSupport {}
impl<'a> Builder<'a> for vk::AttachmentDescription2 {
    type Type = AttachmentDescription2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait AttachmentDescription2Next {}
#[derive(Default)]
pub struct AttachmentDescription2Builder<'a> {
    inner: vk::AttachmentDescription2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> AttachmentDescription2Builder<'a> {
    pub fn insert_next<T: AttachmentDescription2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::AttachmentDescriptionFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn samples(mut self, samples: vk::SampleCountFlags) -> Self {
        self.inner.samples = samples;
        self
    }
    pub fn load_op(mut self, load_op: vk::AttachmentLoadOp) -> Self {
        self.inner.load_op = load_op;
        self
    }
    pub fn store_op(mut self, store_op: vk::AttachmentStoreOp) -> Self {
        self.inner.store_op = store_op;
        self
    }
    pub fn stencil_load_op(mut self, stencil_load_op: vk::AttachmentLoadOp) -> Self {
        self.inner.stencil_load_op = stencil_load_op;
        self
    }
    pub fn stencil_store_op(mut self, stencil_store_op: vk::AttachmentStoreOp) -> Self {
        self.inner.stencil_store_op = stencil_store_op;
        self
    }
    pub fn initial_layout(mut self, initial_layout: vk::ImageLayout) -> Self {
        self.inner.initial_layout = initial_layout;
        self
    }
    pub fn final_layout(mut self, final_layout: vk::ImageLayout) -> Self {
        self.inner.final_layout = final_layout;
        self
    }
}
impl<'a> Deref for AttachmentDescription2Builder<'a> {
    type Target = vk::AttachmentDescription2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AttachmentReference2 {
    type Type = AttachmentReference2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait AttachmentReference2Next {}
#[derive(Default)]
pub struct AttachmentReference2Builder<'a> {
    inner: vk::AttachmentReference2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> AttachmentReference2Builder<'a> {
    pub fn insert_next<T: AttachmentReference2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn attachment(mut self, attachment: u32) -> Self {
        self.inner.attachment = attachment;
        self
    }
    pub fn layout(mut self, layout: vk::ImageLayout) -> Self {
        self.inner.layout = layout;
        self
    }
    pub fn aspect_mask(mut self, aspect_mask: vk::ImageAspectFlags) -> Self {
        self.inner.aspect_mask = aspect_mask;
        self
    }
}
impl<'a> Deref for AttachmentReference2Builder<'a> {
    type Target = vk::AttachmentReference2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SubpassDescription2 {
    type Type = SubpassDescription2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SubpassDescription2Next {}
#[derive(Default)]
pub struct SubpassDescription2Builder<'a> {
    inner: vk::SubpassDescription2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SubpassDescription2Builder<'a> {
    pub fn insert_next<T: SubpassDescription2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SubpassDescriptionFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn view_mask(mut self, view_mask: u32) -> Self {
        self.inner.view_mask = view_mask;
        self
    }
    pub fn p_input_attachments(mut self, p_input_attachments: &'a [vk::AttachmentReference2]) -> Self {
        self.inner.input_attachment_count = p_input_attachments.len() as u32;
        self.inner.p_input_attachments = p_input_attachments.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_color_attachments(
        mut self,
        p_color_attachments: &'a [vk::AttachmentReference2],
        p_resolve_attachments: Option<&'a [vk::AttachmentReference2]>,
    ) -> Self {
        self.inner.color_attachment_count = p_color_attachments.len() as u32;
        if let Some(s) = p_resolve_attachments {
            assert_eq!(self.inner.color_attachment_count, s.len() as u32);
        }
        self.inner.p_color_attachments = p_color_attachments.first().map_or(ptr::null(), |s| s as *const _);
        self.inner.p_resolve_attachments = p_resolve_attachments
            .and_then(|s| s.first())
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_depth_stencil_attachment(
        mut self,
        p_depth_stencil_attachment: Option<&'a vk::AttachmentReference2>,
    ) -> Self {
        self.inner.p_depth_stencil_attachment = p_depth_stencil_attachment.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_preserve_attachments(mut self, p_preserve_attachments: &'a [u32]) -> Self {
        self.inner.preserve_attachment_count = p_preserve_attachments.len() as u32;
        self.inner.p_preserve_attachments = p_preserve_attachments.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for SubpassDescription2Builder<'a> {
    type Target = vk::SubpassDescription2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SubpassDependency2 {
    type Type = SubpassDependency2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SubpassDependency2Next {}
#[derive(Default)]
pub struct SubpassDependency2Builder<'a> {
    inner: vk::SubpassDependency2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SubpassDependency2Builder<'a> {
    pub fn insert_next<T: SubpassDependency2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_subpass(mut self, src_subpass: u32) -> Self {
        self.inner.src_subpass = src_subpass;
        self
    }
    pub fn dst_subpass(mut self, dst_subpass: u32) -> Self {
        self.inner.dst_subpass = dst_subpass;
        self
    }
    pub fn src_stage_mask(mut self, src_stage_mask: vk::PipelineStageFlags) -> Self {
        self.inner.src_stage_mask = src_stage_mask;
        self
    }
    pub fn dst_stage_mask(mut self, dst_stage_mask: vk::PipelineStageFlags) -> Self {
        self.inner.dst_stage_mask = dst_stage_mask;
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: vk::AccessFlags) -> Self {
        self.inner.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: vk::AccessFlags) -> Self {
        self.inner.dst_access_mask = dst_access_mask;
        self
    }
    pub fn dependency_flags(mut self, dependency_flags: vk::DependencyFlags) -> Self {
        self.inner.dependency_flags = dependency_flags;
        self
    }
    pub fn view_offset(mut self, view_offset: i32) -> Self {
        self.inner.view_offset = view_offset;
        self
    }
}
impl<'a> Deref for SubpassDependency2Builder<'a> {
    type Target = vk::SubpassDependency2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RenderPassCreateInfo2 {
    type Type = RenderPassCreateInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait RenderPassCreateInfo2Next {}
#[derive(Default)]
pub struct RenderPassCreateInfo2Builder<'a> {
    inner: vk::RenderPassCreateInfo2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RenderPassCreateInfo2Builder<'a> {
    pub fn insert_next<T: RenderPassCreateInfo2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::RenderPassCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_attachments(mut self, p_attachments: &'a [vk::AttachmentDescription2]) -> Self {
        self.inner.attachment_count = p_attachments.len() as u32;
        self.inner.p_attachments = p_attachments.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_subpasses(mut self, p_subpasses: &'a [vk::SubpassDescription2]) -> Self {
        self.inner.subpass_count = p_subpasses.len() as u32;
        self.inner.p_subpasses = p_subpasses.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_dependencies(mut self, p_dependencies: &'a [vk::SubpassDependency2]) -> Self {
        self.inner.dependency_count = p_dependencies.len() as u32;
        self.inner.p_dependencies = p_dependencies.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_correlated_view_masks(mut self, p_correlated_view_masks: &'a [u32]) -> Self {
        self.inner.correlated_view_mask_count = p_correlated_view_masks.len() as u32;
        self.inner.p_correlated_view_masks = p_correlated_view_masks.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for RenderPassCreateInfo2Builder<'a> {
    type Target = vk::RenderPassCreateInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SubpassBeginInfo {
    type Type = SubpassBeginInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubpassBeginInfoBuilder {
    inner: vk::SubpassBeginInfo,
}
impl SubpassBeginInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn contents(mut self, contents: vk::SubpassContents) -> Self {
        self.inner.contents = contents;
        self
    }
}
impl Deref for SubpassBeginInfoBuilder {
    type Target = vk::SubpassBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SubpassEndInfo {
    type Type = SubpassEndInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SubpassEndInfoNext {}
#[derive(Default)]
pub struct SubpassEndInfoBuilder<'a> {
    inner: vk::SubpassEndInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SubpassEndInfoBuilder<'a> {
    pub fn insert_next<T: SubpassEndInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
}
impl<'a> Deref for SubpassEndInfoBuilder<'a> {
    type Target = vk::SubpassEndInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceTimelineSemaphoreFeatures {
    type Type = PhysicalDeviceTimelineSemaphoreFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceTimelineSemaphoreFeaturesBuilder {
    inner: vk::PhysicalDeviceTimelineSemaphoreFeatures,
}
impl PhysicalDeviceTimelineSemaphoreFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn timeline_semaphore(mut self, timeline_semaphore: bool) -> Self {
        self.inner.timeline_semaphore = if timeline_semaphore { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceTimelineSemaphoreFeaturesBuilder {
    type Target = vk::PhysicalDeviceTimelineSemaphoreFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceTimelineSemaphoreFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceTimelineSemaphoreFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceTimelineSemaphoreFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceTimelineSemaphoreFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceTimelineSemaphoreProperties {}
impl Builder<'_> for vk::SemaphoreTypeCreateInfo {
    type Type = SemaphoreTypeCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreTypeCreateInfoBuilder {
    inner: vk::SemaphoreTypeCreateInfo,
}
impl SemaphoreTypeCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore_type(mut self, semaphore_type: vk::SemaphoreType) -> Self {
        self.inner.semaphore_type = semaphore_type;
        self
    }
    pub fn initial_value(mut self, initial_value: u64) -> Self {
        self.inner.initial_value = initial_value;
        self
    }
}
impl Deref for SemaphoreTypeCreateInfoBuilder {
    type Target = vk::SemaphoreTypeCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SemaphoreCreateInfoNext for SemaphoreTypeCreateInfoBuilder {}
impl PhysicalDeviceExternalSemaphoreInfoNext for SemaphoreTypeCreateInfoBuilder {}
impl SemaphoreCreateInfoNext for vk::SemaphoreTypeCreateInfo {}
impl PhysicalDeviceExternalSemaphoreInfoNext for vk::SemaphoreTypeCreateInfo {}
impl<'a> Builder<'a> for vk::TimelineSemaphoreSubmitInfo {
    type Type = TimelineSemaphoreSubmitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct TimelineSemaphoreSubmitInfoBuilder<'a> {
    inner: vk::TimelineSemaphoreSubmitInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> TimelineSemaphoreSubmitInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphore_values(mut self, p_wait_semaphore_values: &'a [u64]) -> Self {
        self.inner.wait_semaphore_value_count = p_wait_semaphore_values.len() as u32;
        self.inner.p_wait_semaphore_values = p_wait_semaphore_values.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_signal_semaphore_values(mut self, p_signal_semaphore_values: &'a [u64]) -> Self {
        self.inner.signal_semaphore_value_count = p_signal_semaphore_values.len() as u32;
        self.inner.p_signal_semaphore_values = p_signal_semaphore_values.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for TimelineSemaphoreSubmitInfoBuilder<'a> {
    type Target = vk::TimelineSemaphoreSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> SubmitInfoNext for TimelineSemaphoreSubmitInfoBuilder<'a> {}
impl<'a> BindSparseInfoNext for TimelineSemaphoreSubmitInfoBuilder<'a> {}
impl SubmitInfoNext for vk::TimelineSemaphoreSubmitInfo {}
impl BindSparseInfoNext for vk::TimelineSemaphoreSubmitInfo {}
impl<'a> Builder<'a> for vk::SemaphoreWaitInfo {
    type Type = SemaphoreWaitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreWaitInfoBuilder<'a> {
    inner: vk::SemaphoreWaitInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SemaphoreWaitInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SemaphoreWaitFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_semaphores(mut self, p_semaphores: &'a [vk::Semaphore], p_values: &'a [u64]) -> Self {
        self.inner.semaphore_count = p_semaphores.len() as u32;
        assert_eq!(self.inner.semaphore_count, p_values.len() as u32);
        self.inner.p_semaphores = p_semaphores.first().map_or(ptr::null(), |s| s as *const _);
        self.inner.p_values = p_values.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for SemaphoreWaitInfoBuilder<'a> {
    type Target = vk::SemaphoreWaitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SemaphoreSignalInfo {
    type Type = SemaphoreSignalInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreSignalInfoBuilder {
    inner: vk::SemaphoreSignalInfo,
}
impl SemaphoreSignalInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn value(mut self, value: u64) -> Self {
        self.inner.value = value;
        self
    }
}
impl Deref for SemaphoreSignalInfoBuilder {
    type Target = vk::SemaphoreSignalInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineVertexInputDivisorStateCreateInfoEXT {
    type Type = PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineVertexInputDivisorStateCreateInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_vertex_binding_divisors(
        mut self,
        p_vertex_binding_divisors: &'a [vk::VertexInputBindingDivisorDescriptionEXT],
    ) -> Self {
        self.inner.vertex_binding_divisor_count = p_vertex_binding_divisors.len() as u32;
        self.inner.p_vertex_binding_divisors = p_vertex_binding_divisors.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineVertexInputDivisorStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> PipelineVertexInputStateCreateInfoNext for PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {}
impl PipelineVertexInputStateCreateInfoNext for vk::PipelineVertexInputDivisorStateCreateInfoEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceVertexAttributeDivisorPropertiesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDevicePCIBusInfoPropertiesEXT {}
impl Builder<'_> for vk::ImportAndroidHardwareBufferInfoANDROID {
    type Type = ImportAndroidHardwareBufferInfoANDROIDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportAndroidHardwareBufferInfoANDROIDBuilder {
    inner: vk::ImportAndroidHardwareBufferInfoANDROID,
}
impl ImportAndroidHardwareBufferInfoANDROIDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer(mut self, buffer: *mut vk::AHardwareBuffer) -> Self {
        self.inner.buffer = buffer;
        self
    }
}
impl Deref for ImportAndroidHardwareBufferInfoANDROIDBuilder {
    type Target = vk::ImportAndroidHardwareBufferInfoANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for ImportAndroidHardwareBufferInfoANDROIDBuilder {}
impl MemoryAllocateInfoNext for vk::ImportAndroidHardwareBufferInfoANDROID {}
impl ImageFormatProperties2Next for vk::AndroidHardwareBufferUsageANDROID {}
impl<'a> Builder<'a> for vk::AndroidHardwareBufferPropertiesANDROID {
    type Type = AndroidHardwareBufferPropertiesANDROIDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait AndroidHardwareBufferPropertiesANDROIDNext {}
#[derive(Default)]
pub struct AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
    inner: vk::AndroidHardwareBufferPropertiesANDROID,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
    pub fn insert_next<T: AndroidHardwareBufferPropertiesANDROIDNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn get_mut(&mut self) -> &mut vk::AndroidHardwareBufferPropertiesANDROID {
        &mut self.inner
    }
}
impl<'a> Deref for AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
    type Target = vk::AndroidHardwareBufferPropertiesANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::MemoryGetAndroidHardwareBufferInfoANDROID {
    type Type = MemoryGetAndroidHardwareBufferInfoANDROIDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROIDBuilder {
    inner: vk::MemoryGetAndroidHardwareBufferInfoANDROID,
}
impl MemoryGetAndroidHardwareBufferInfoANDROIDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
}
impl Deref for MemoryGetAndroidHardwareBufferInfoANDROIDBuilder {
    type Target = vk::MemoryGetAndroidHardwareBufferInfoANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl AndroidHardwareBufferPropertiesANDROIDNext for vk::AndroidHardwareBufferFormatPropertiesANDROID {}
impl Builder<'_> for vk::CommandBufferInheritanceConditionalRenderingInfoEXT {
    type Type = CommandBufferInheritanceConditionalRenderingInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXTBuilder {
    inner: vk::CommandBufferInheritanceConditionalRenderingInfoEXT,
}
impl CommandBufferInheritanceConditionalRenderingInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn conditional_rendering_enable(mut self, conditional_rendering_enable: bool) -> Self {
        self.inner.conditional_rendering_enable = if conditional_rendering_enable {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for CommandBufferInheritanceConditionalRenderingInfoEXTBuilder {
    type Target = vk::CommandBufferInheritanceConditionalRenderingInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl CommandBufferInheritanceInfoNext for CommandBufferInheritanceConditionalRenderingInfoEXTBuilder {}
impl CommandBufferInheritanceInfoNext for vk::CommandBufferInheritanceConditionalRenderingInfoEXT {}
impl Builder<'_> for vk::ExternalFormatANDROID {
    type Type = ExternalFormatANDROIDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExternalFormatANDROIDBuilder {
    inner: vk::ExternalFormatANDROID,
}
impl ExternalFormatANDROIDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn external_format(mut self, external_format: u64) -> Self {
        self.inner.external_format = external_format;
        self
    }
}
impl Deref for ExternalFormatANDROIDBuilder {
    type Target = vk::ExternalFormatANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageCreateInfoNext for ExternalFormatANDROIDBuilder {}
impl SamplerYcbcrConversionCreateInfoNext for ExternalFormatANDROIDBuilder {}
impl ImageCreateInfoNext for vk::ExternalFormatANDROID {}
impl SamplerYcbcrConversionCreateInfoNext for vk::ExternalFormatANDROID {}
impl Builder<'_> for vk::PhysicalDevice8BitStorageFeatures {
    type Type = PhysicalDevice8BitStorageFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevice8BitStorageFeaturesBuilder {
    inner: vk::PhysicalDevice8BitStorageFeatures,
}
impl PhysicalDevice8BitStorageFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn storage_buffer8_bit_access(mut self, storage_buffer8_bit_access: bool) -> Self {
        self.inner.storage_buffer8_bit_access = if storage_buffer8_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn uniform_and_storage_buffer8_bit_access(mut self, uniform_and_storage_buffer8_bit_access: bool) -> Self {
        self.inner.uniform_and_storage_buffer8_bit_access = if uniform_and_storage_buffer8_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn storage_push_constant8(mut self, storage_push_constant8: bool) -> Self {
        self.inner.storage_push_constant8 = if storage_push_constant8 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDevice8BitStorageFeaturesBuilder {
    type Target = vk::PhysicalDevice8BitStorageFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDevice8BitStorageFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDevice8BitStorageFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevice8BitStorageFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDevice8BitStorageFeatures {}
impl Builder<'_> for vk::PhysicalDeviceConditionalRenderingFeaturesEXT {
    type Type = PhysicalDeviceConditionalRenderingFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceConditionalRenderingFeaturesEXT,
}
impl PhysicalDeviceConditionalRenderingFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn conditional_rendering(mut self, conditional_rendering: bool) -> Self {
        self.inner.conditional_rendering = if conditional_rendering { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn inherited_conditional_rendering(mut self, inherited_conditional_rendering: bool) -> Self {
        self.inner.inherited_conditional_rendering = if inherited_conditional_rendering {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceConditionalRenderingFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceConditionalRenderingFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceConditionalRenderingFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceConditionalRenderingFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceConditionalRenderingFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceConditionalRenderingFeaturesEXT {}
impl Builder<'_> for vk::PhysicalDeviceVulkanMemoryModelFeatures {
    type Type = PhysicalDeviceVulkanMemoryModelFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVulkanMemoryModelFeaturesBuilder {
    inner: vk::PhysicalDeviceVulkanMemoryModelFeatures,
}
impl PhysicalDeviceVulkanMemoryModelFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn vulkan_memory_model(mut self, vulkan_memory_model: bool) -> Self {
        self.inner.vulkan_memory_model = if vulkan_memory_model { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn vulkan_memory_model_device_scope(mut self, vulkan_memory_model_device_scope: bool) -> Self {
        self.inner.vulkan_memory_model_device_scope = if vulkan_memory_model_device_scope {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn vulkan_memory_model_availability_visibility_chains(
        mut self,
        vulkan_memory_model_availability_visibility_chains: bool,
    ) -> Self {
        self.inner.vulkan_memory_model_availability_visibility_chains =
            if vulkan_memory_model_availability_visibility_chains {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
}
impl Deref for PhysicalDeviceVulkanMemoryModelFeaturesBuilder {
    type Target = vk::PhysicalDeviceVulkanMemoryModelFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceVulkanMemoryModelFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceVulkanMemoryModelFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceVulkanMemoryModelFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceVulkanMemoryModelFeatures {}
impl Builder<'_> for vk::PhysicalDeviceShaderAtomicInt64Features {
    type Type = PhysicalDeviceShaderAtomicInt64FeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderAtomicInt64FeaturesBuilder {
    inner: vk::PhysicalDeviceShaderAtomicInt64Features,
}
impl PhysicalDeviceShaderAtomicInt64FeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_buffer_int64_atomics(mut self, shader_buffer_int64_atomics: bool) -> Self {
        self.inner.shader_buffer_int64_atomics = if shader_buffer_int64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_int64_atomics(mut self, shader_shared_int64_atomics: bool) -> Self {
        self.inner.shader_shared_int64_atomics = if shader_shared_int64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShaderAtomicInt64FeaturesBuilder {
    type Target = vk::PhysicalDeviceShaderAtomicInt64Features;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceShaderAtomicInt64FeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceShaderAtomicInt64FeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderAtomicInt64Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderAtomicInt64Features {}
impl Builder<'_> for vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    type Type = PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT,
}
impl PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_buffer_float32_atomics(mut self, shader_buffer_float32_atomics: bool) -> Self {
        self.inner.shader_buffer_float32_atomics = if shader_buffer_float32_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_buffer_float32_atomic_add(mut self, shader_buffer_float32_atomic_add: bool) -> Self {
        self.inner.shader_buffer_float32_atomic_add = if shader_buffer_float32_atomic_add {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_buffer_float64_atomics(mut self, shader_buffer_float64_atomics: bool) -> Self {
        self.inner.shader_buffer_float64_atomics = if shader_buffer_float64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_buffer_float64_atomic_add(mut self, shader_buffer_float64_atomic_add: bool) -> Self {
        self.inner.shader_buffer_float64_atomic_add = if shader_buffer_float64_atomic_add {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_float32_atomics(mut self, shader_shared_float32_atomics: bool) -> Self {
        self.inner.shader_shared_float32_atomics = if shader_shared_float32_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_float32_atomic_add(mut self, shader_shared_float32_atomic_add: bool) -> Self {
        self.inner.shader_shared_float32_atomic_add = if shader_shared_float32_atomic_add {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_float64_atomics(mut self, shader_shared_float64_atomics: bool) -> Self {
        self.inner.shader_shared_float64_atomics = if shader_shared_float64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_float64_atomic_add(mut self, shader_shared_float64_atomic_add: bool) -> Self {
        self.inner.shader_shared_float64_atomic_add = if shader_shared_float64_atomic_add {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_image_float32_atomics(mut self, shader_image_float32_atomics: bool) -> Self {
        self.inner.shader_image_float32_atomics = if shader_image_float32_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_image_float32_atomic_add(mut self, shader_image_float32_atomic_add: bool) -> Self {
        self.inner.shader_image_float32_atomic_add = if shader_image_float32_atomic_add {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn sparse_image_float32_atomics(mut self, sparse_image_float32_atomics: bool) -> Self {
        self.inner.sparse_image_float32_atomics = if sparse_image_float32_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn sparse_image_float32_atomic_add(mut self, sparse_image_float32_atomic_add: bool) -> Self {
        self.inner.sparse_image_float32_atomic_add = if sparse_image_float32_atomic_add {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT {}
impl Builder<'_> for vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    type Type = PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder {
    inner: vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT,
}
impl PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_buffer_float16_atomics(mut self, shader_buffer_float16_atomics: bool) -> Self {
        self.inner.shader_buffer_float16_atomics = if shader_buffer_float16_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_buffer_float16_atomic_add(mut self, shader_buffer_float16_atomic_add: bool) -> Self {
        self.inner.shader_buffer_float16_atomic_add = if shader_buffer_float16_atomic_add {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_buffer_float16_atomic_min_max(mut self, shader_buffer_float16_atomic_min_max: bool) -> Self {
        self.inner.shader_buffer_float16_atomic_min_max = if shader_buffer_float16_atomic_min_max {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_buffer_float32_atomic_min_max(mut self, shader_buffer_float32_atomic_min_max: bool) -> Self {
        self.inner.shader_buffer_float32_atomic_min_max = if shader_buffer_float32_atomic_min_max {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_buffer_float64_atomic_min_max(mut self, shader_buffer_float64_atomic_min_max: bool) -> Self {
        self.inner.shader_buffer_float64_atomic_min_max = if shader_buffer_float64_atomic_min_max {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_float16_atomics(mut self, shader_shared_float16_atomics: bool) -> Self {
        self.inner.shader_shared_float16_atomics = if shader_shared_float16_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_float16_atomic_add(mut self, shader_shared_float16_atomic_add: bool) -> Self {
        self.inner.shader_shared_float16_atomic_add = if shader_shared_float16_atomic_add {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_float16_atomic_min_max(mut self, shader_shared_float16_atomic_min_max: bool) -> Self {
        self.inner.shader_shared_float16_atomic_min_max = if shader_shared_float16_atomic_min_max {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_float32_atomic_min_max(mut self, shader_shared_float32_atomic_min_max: bool) -> Self {
        self.inner.shader_shared_float32_atomic_min_max = if shader_shared_float32_atomic_min_max {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_float64_atomic_min_max(mut self, shader_shared_float64_atomic_min_max: bool) -> Self {
        self.inner.shader_shared_float64_atomic_min_max = if shader_shared_float64_atomic_min_max {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_image_float32_atomic_min_max(mut self, shader_image_float32_atomic_min_max: bool) -> Self {
        self.inner.shader_image_float32_atomic_min_max = if shader_image_float32_atomic_min_max {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn sparse_image_float32_atomic_min_max(mut self, sparse_image_float32_atomic_min_max: bool) -> Self {
        self.inner.sparse_image_float32_atomic_min_max = if sparse_image_float32_atomic_min_max {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceShaderAtomicFloat2FeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT {}
impl Builder<'_> for vk::PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    type Type = PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceVertexAttributeDivisorFeaturesEXT,
}
impl PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn vertex_attribute_instance_rate_divisor(mut self, vertex_attribute_instance_rate_divisor: bool) -> Self {
        self.inner.vertex_attribute_instance_rate_divisor = if vertex_attribute_instance_rate_divisor {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn vertex_attribute_instance_rate_zero_divisor(
        mut self,
        vertex_attribute_instance_rate_zero_divisor: bool,
    ) -> Self {
        self.inner.vertex_attribute_instance_rate_zero_divisor = if vertex_attribute_instance_rate_zero_divisor {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceVertexAttributeDivisorFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceVertexAttributeDivisorFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceVertexAttributeDivisorFeaturesEXT {}
impl QueueFamilyProperties2Next for vk::QueueFamilyCheckpointPropertiesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDepthStencilResolveProperties {}
impl<'a> Builder<'a> for vk::SubpassDescriptionDepthStencilResolve {
    type Type = SubpassDescriptionDepthStencilResolveBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubpassDescriptionDepthStencilResolveBuilder<'a> {
    inner: vk::SubpassDescriptionDepthStencilResolve,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SubpassDescriptionDepthStencilResolveBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn depth_resolve_mode(mut self, depth_resolve_mode: vk::ResolveModeFlags) -> Self {
        self.inner.depth_resolve_mode = depth_resolve_mode;
        self
    }
    pub fn stencil_resolve_mode(mut self, stencil_resolve_mode: vk::ResolveModeFlags) -> Self {
        self.inner.stencil_resolve_mode = stencil_resolve_mode;
        self
    }
    pub fn p_depth_stencil_resolve_attachment(
        mut self,
        p_depth_stencil_resolve_attachment: Option<&'a vk::AttachmentReference2>,
    ) -> Self {
        self.inner.p_depth_stencil_resolve_attachment = p_depth_stencil_resolve_attachment.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for SubpassDescriptionDepthStencilResolveBuilder<'a> {
    type Target = vk::SubpassDescriptionDepthStencilResolve;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> SubpassDescription2Next for SubpassDescriptionDepthStencilResolveBuilder<'a> {}
impl SubpassDescription2Next for vk::SubpassDescriptionDepthStencilResolve {}
impl Builder<'_> for vk::ImageViewASTCDecodeModeEXT {
    type Type = ImageViewASTCDecodeModeEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageViewASTCDecodeModeEXTBuilder {
    inner: vk::ImageViewASTCDecodeModeEXT,
}
impl ImageViewASTCDecodeModeEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn decode_mode(mut self, decode_mode: vk::Format) -> Self {
        self.inner.decode_mode = decode_mode;
        self
    }
}
impl Deref for ImageViewASTCDecodeModeEXTBuilder {
    type Target = vk::ImageViewASTCDecodeModeEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageViewCreateInfoNext for ImageViewASTCDecodeModeEXTBuilder {}
impl ImageViewCreateInfoNext for vk::ImageViewASTCDecodeModeEXT {}
impl Builder<'_> for vk::PhysicalDeviceASTCDecodeFeaturesEXT {
    type Type = PhysicalDeviceASTCDecodeFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceASTCDecodeFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceASTCDecodeFeaturesEXT,
}
impl PhysicalDeviceASTCDecodeFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn decode_mode_shared_exponent(mut self, decode_mode_shared_exponent: bool) -> Self {
        self.inner.decode_mode_shared_exponent = if decode_mode_shared_exponent {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceASTCDecodeFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceASTCDecodeFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceASTCDecodeFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceASTCDecodeFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceASTCDecodeFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceASTCDecodeFeaturesEXT {}
impl Builder<'_> for vk::PhysicalDeviceTransformFeedbackFeaturesEXT {
    type Type = PhysicalDeviceTransformFeedbackFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceTransformFeedbackFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceTransformFeedbackFeaturesEXT,
}
impl PhysicalDeviceTransformFeedbackFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn transform_feedback(mut self, transform_feedback: bool) -> Self {
        self.inner.transform_feedback = if transform_feedback { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn geometry_streams(mut self, geometry_streams: bool) -> Self {
        self.inner.geometry_streams = if geometry_streams { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceTransformFeedbackFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceTransformFeedbackFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceTransformFeedbackFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceTransformFeedbackFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceTransformFeedbackFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceTransformFeedbackFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceTransformFeedbackPropertiesEXT {}
impl Builder<'_> for vk::PipelineRasterizationStateStreamCreateInfoEXT {
    type Type = PipelineRasterizationStateStreamCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationStateStreamCreateInfoEXTBuilder {
    inner: vk::PipelineRasterizationStateStreamCreateInfoEXT,
}
impl PipelineRasterizationStateStreamCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineRasterizationStateStreamCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn rasterization_stream(mut self, rasterization_stream: u32) -> Self {
        self.inner.rasterization_stream = rasterization_stream;
        self
    }
}
impl Deref for PipelineRasterizationStateStreamCreateInfoEXTBuilder {
    type Target = vk::PipelineRasterizationStateStreamCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineRasterizationStateCreateInfoNext for PipelineRasterizationStateStreamCreateInfoEXTBuilder {}
impl PipelineRasterizationStateCreateInfoNext for vk::PipelineRasterizationStateStreamCreateInfoEXT {}
impl Builder<'_> for vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    type Type = PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder {
    inner: vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV,
}
impl PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn representative_fragment_test(mut self, representative_fragment_test: bool) -> Self {
        self.inner.representative_fragment_test = if representative_fragment_test {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV {}
impl Builder<'_> for vk::PipelineRepresentativeFragmentTestStateCreateInfoNV {
    type Type = PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder {
    inner: vk::PipelineRepresentativeFragmentTestStateCreateInfoNV,
}
impl PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn representative_fragment_test_enable(mut self, representative_fragment_test_enable: bool) -> Self {
        self.inner.representative_fragment_test_enable = if representative_fragment_test_enable {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder {
    type Target = vk::PipelineRepresentativeFragmentTestStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl GraphicsPipelineCreateInfoNext for PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder {}
impl GraphicsPipelineCreateInfoNext for vk::PipelineRepresentativeFragmentTestStateCreateInfoNV {}
impl Builder<'_> for vk::PhysicalDeviceExclusiveScissorFeaturesNV {
    type Type = PhysicalDeviceExclusiveScissorFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExclusiveScissorFeaturesNVBuilder {
    inner: vk::PhysicalDeviceExclusiveScissorFeaturesNV,
}
impl PhysicalDeviceExclusiveScissorFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn exclusive_scissor(mut self, exclusive_scissor: bool) -> Self {
        self.inner.exclusive_scissor = if exclusive_scissor { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceExclusiveScissorFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceExclusiveScissorFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceExclusiveScissorFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceExclusiveScissorFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceExclusiveScissorFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceExclusiveScissorFeaturesNV {}
impl<'a> Builder<'a> for vk::PipelineViewportExclusiveScissorStateCreateInfoNV {
    type Type = PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportExclusiveScissorStateCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_exclusive_scissors(mut self, p_exclusive_scissors: &'a [vk::Rect2D]) -> Self {
        self.inner.exclusive_scissor_count = p_exclusive_scissors.len() as u32;
        self.inner.p_exclusive_scissors = p_exclusive_scissors.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineViewportExclusiveScissorStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> PipelineViewportStateCreateInfoNext for PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {}
impl PipelineViewportStateCreateInfoNext for vk::PipelineViewportExclusiveScissorStateCreateInfoNV {}
impl Builder<'_> for vk::PhysicalDeviceCornerSampledImageFeaturesNV {
    type Type = PhysicalDeviceCornerSampledImageFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceCornerSampledImageFeaturesNVBuilder {
    inner: vk::PhysicalDeviceCornerSampledImageFeaturesNV,
}
impl PhysicalDeviceCornerSampledImageFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn corner_sampled_image(mut self, corner_sampled_image: bool) -> Self {
        self.inner.corner_sampled_image = if corner_sampled_image { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceCornerSampledImageFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceCornerSampledImageFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceCornerSampledImageFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceCornerSampledImageFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCornerSampledImageFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCornerSampledImageFeaturesNV {}
impl Builder<'_> for vk::PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    type Type = PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder {
    inner: vk::PhysicalDeviceComputeShaderDerivativesFeaturesNV,
}
impl PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn compute_derivative_group_quads(mut self, compute_derivative_group_quads: bool) -> Self {
        self.inner.compute_derivative_group_quads = if compute_derivative_group_quads {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn compute_derivative_group_linear(mut self, compute_derivative_group_linear: bool) -> Self {
        self.inner.compute_derivative_group_linear = if compute_derivative_group_linear {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceComputeShaderDerivativesFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceComputeShaderDerivativesFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceComputeShaderDerivativesFeaturesNV {}
impl Builder<'_> for vk::PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    type Type = PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder {
    inner: vk::PhysicalDeviceFragmentShaderBarycentricFeaturesNV,
}
impl PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_shader_barycentric(mut self, fragment_shader_barycentric: bool) -> Self {
        self.inner.fragment_shader_barycentric = if fragment_shader_barycentric {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceFragmentShaderBarycentricFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFragmentShaderBarycentricFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFragmentShaderBarycentricFeaturesNV {}
impl Builder<'_> for vk::PhysicalDeviceShaderImageFootprintFeaturesNV {
    type Type = PhysicalDeviceShaderImageFootprintFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderImageFootprintFeaturesNVBuilder {
    inner: vk::PhysicalDeviceShaderImageFootprintFeaturesNV,
}
impl PhysicalDeviceShaderImageFootprintFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image_footprint(mut self, image_footprint: bool) -> Self {
        self.inner.image_footprint = if image_footprint { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceShaderImageFootprintFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceShaderImageFootprintFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceShaderImageFootprintFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceShaderImageFootprintFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderImageFootprintFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderImageFootprintFeaturesNV {}
impl Builder<'_> for vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    type Type = PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder {
    inner: vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV,
}
impl PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dedicated_allocation_image_aliasing(mut self, dedicated_allocation_image_aliasing: bool) -> Self {
        self.inner.dedicated_allocation_image_aliasing = if dedicated_allocation_image_aliasing {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {}
impl<'a> Builder<'a> for vk::ShadingRatePaletteNV {
    type Type = ShadingRatePaletteNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ShadingRatePaletteNVBuilder<'a> {
    inner: vk::ShadingRatePaletteNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ShadingRatePaletteNVBuilder<'a> {
    pub fn p_shading_rate_palette_entries(
        mut self,
        p_shading_rate_palette_entries: &'a [vk::ShadingRatePaletteEntryNV],
    ) -> Self {
        self.inner.shading_rate_palette_entry_count = p_shading_rate_palette_entries.len() as u32;
        self.inner.p_shading_rate_palette_entries = p_shading_rate_palette_entries
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for ShadingRatePaletteNVBuilder<'a> {
    type Target = vk::ShadingRatePaletteNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineViewportShadingRateImageStateCreateInfoNV {
    type Type = PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportShadingRateImageStateCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shading_rate_image_enable(mut self, shading_rate_image_enable: bool) -> Self {
        self.inner.shading_rate_image_enable = if shading_rate_image_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn p_shading_rate_palettes(mut self, p_shading_rate_palettes: &'a [vk::ShadingRatePaletteNV]) -> Self {
        self.inner.viewport_count = p_shading_rate_palettes.len() as u32;
        self.inner.p_shading_rate_palettes = p_shading_rate_palettes.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineViewportShadingRateImageStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> PipelineViewportStateCreateInfoNext for PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {}
impl PipelineViewportStateCreateInfoNext for vk::PipelineViewportShadingRateImageStateCreateInfoNV {}
impl Builder<'_> for vk::PhysicalDeviceShadingRateImageFeaturesNV {
    type Type = PhysicalDeviceShadingRateImageFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShadingRateImageFeaturesNVBuilder {
    inner: vk::PhysicalDeviceShadingRateImageFeaturesNV,
}
impl PhysicalDeviceShadingRateImageFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shading_rate_image(mut self, shading_rate_image: bool) -> Self {
        self.inner.shading_rate_image = if shading_rate_image { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shading_rate_coarse_sample_order(mut self, shading_rate_coarse_sample_order: bool) -> Self {
        self.inner.shading_rate_coarse_sample_order = if shading_rate_coarse_sample_order {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShadingRateImageFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceShadingRateImageFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceShadingRateImageFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceShadingRateImageFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShadingRateImageFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShadingRateImageFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceShadingRateImagePropertiesNV {}
impl Builder<'_> for vk::PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    type Type = PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder {
    inner: vk::PhysicalDeviceInvocationMaskFeaturesHUAWEI,
}
impl PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn invocation_mask(mut self, invocation_mask: bool) -> Self {
        self.inner.invocation_mask = if invocation_mask { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder {
    type Target = vk::PhysicalDeviceInvocationMaskFeaturesHUAWEI;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceInvocationMaskFeaturesHUAWEI {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceInvocationMaskFeaturesHUAWEI {}
impl<'a> Builder<'a> for vk::CoarseSampleOrderCustomNV {
    type Type = CoarseSampleOrderCustomNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CoarseSampleOrderCustomNVBuilder<'a> {
    inner: vk::CoarseSampleOrderCustomNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CoarseSampleOrderCustomNVBuilder<'a> {
    pub fn shading_rate(mut self, shading_rate: vk::ShadingRatePaletteEntryNV) -> Self {
        self.inner.shading_rate = shading_rate;
        self
    }
    pub fn sample_count(mut self, sample_count: u32) -> Self {
        self.inner.sample_count = sample_count;
        self
    }
    pub fn p_sample_locations(mut self, p_sample_locations: &'a [vk::CoarseSampleLocationNV]) -> Self {
        self.inner.sample_location_count = p_sample_locations.len() as u32;
        self.inner.p_sample_locations = p_sample_locations.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for CoarseSampleOrderCustomNVBuilder<'a> {
    type Target = vk::CoarseSampleOrderCustomNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    type Type = PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportCoarseSampleOrderStateCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn sample_order_type(mut self, sample_order_type: vk::CoarseSampleOrderTypeNV) -> Self {
        self.inner.sample_order_type = sample_order_type;
        self
    }
    pub fn p_custom_sample_orders(mut self, p_custom_sample_orders: &'a [vk::CoarseSampleOrderCustomNV]) -> Self {
        self.inner.custom_sample_order_count = p_custom_sample_orders.len() as u32;
        self.inner.p_custom_sample_orders = p_custom_sample_orders.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineViewportCoarseSampleOrderStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> PipelineViewportStateCreateInfoNext for PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {}
impl PipelineViewportStateCreateInfoNext for vk::PipelineViewportCoarseSampleOrderStateCreateInfoNV {}
impl Builder<'_> for vk::PhysicalDeviceMeshShaderFeaturesNV {
    type Type = PhysicalDeviceMeshShaderFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceMeshShaderFeaturesNVBuilder {
    inner: vk::PhysicalDeviceMeshShaderFeaturesNV,
}
impl PhysicalDeviceMeshShaderFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn task_shader(mut self, task_shader: bool) -> Self {
        self.inner.task_shader = if task_shader { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn mesh_shader(mut self, mesh_shader: bool) -> Self {
        self.inner.mesh_shader = if mesh_shader { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceMeshShaderFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceMeshShaderFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceMeshShaderFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceMeshShaderFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMeshShaderFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMeshShaderFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMeshShaderPropertiesNV {}
impl Builder<'_> for vk::RayTracingShaderGroupCreateInfoNV {
    type Type = RayTracingShaderGroupCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RayTracingShaderGroupCreateInfoNVBuilder {
    inner: vk::RayTracingShaderGroupCreateInfoNV,
}
impl RayTracingShaderGroupCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::RayTracingShaderGroupTypeKHR) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn general_shader(mut self, general_shader: u32) -> Self {
        self.inner.general_shader = general_shader;
        self
    }
    pub fn closest_hit_shader(mut self, closest_hit_shader: u32) -> Self {
        self.inner.closest_hit_shader = closest_hit_shader;
        self
    }
    pub fn any_hit_shader(mut self, any_hit_shader: u32) -> Self {
        self.inner.any_hit_shader = any_hit_shader;
        self
    }
    pub fn intersection_shader(mut self, intersection_shader: u32) -> Self {
        self.inner.intersection_shader = intersection_shader;
        self
    }
}
impl Deref for RayTracingShaderGroupCreateInfoNVBuilder {
    type Target = vk::RayTracingShaderGroupCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::RayTracingShaderGroupCreateInfoKHR {
    type Type = RayTracingShaderGroupCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RayTracingShaderGroupCreateInfoKHRBuilder {
    inner: vk::RayTracingShaderGroupCreateInfoKHR,
}
impl RayTracingShaderGroupCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::RayTracingShaderGroupTypeKHR) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn general_shader(mut self, general_shader: u32) -> Self {
        self.inner.general_shader = general_shader;
        self
    }
    pub fn closest_hit_shader(mut self, closest_hit_shader: u32) -> Self {
        self.inner.closest_hit_shader = closest_hit_shader;
        self
    }
    pub fn any_hit_shader(mut self, any_hit_shader: u32) -> Self {
        self.inner.any_hit_shader = any_hit_shader;
        self
    }
    pub fn intersection_shader(mut self, intersection_shader: u32) -> Self {
        self.inner.intersection_shader = intersection_shader;
        self
    }
    pub fn p_shader_group_capture_replay_handle(mut self, p_shader_group_capture_replay_handle: *const c_void) -> Self {
        self.inner.p_shader_group_capture_replay_handle = p_shader_group_capture_replay_handle;
        self
    }
}
impl Deref for RayTracingShaderGroupCreateInfoKHRBuilder {
    type Target = vk::RayTracingShaderGroupCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RayTracingPipelineCreateInfoNV {
    type Type = RayTracingPipelineCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait RayTracingPipelineCreateInfoNVNext {}
#[derive(Default)]
pub struct RayTracingPipelineCreateInfoNVBuilder<'a> {
    inner: vk::RayTracingPipelineCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RayTracingPipelineCreateInfoNVBuilder<'a> {
    pub fn insert_next<T: RayTracingPipelineCreateInfoNVNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_stages(mut self, p_stages: &'a [vk::PipelineShaderStageCreateInfo]) -> Self {
        self.inner.stage_count = p_stages.len() as u32;
        self.inner.p_stages = p_stages.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_groups(mut self, p_groups: &'a [vk::RayTracingShaderGroupCreateInfoNV]) -> Self {
        self.inner.group_count = p_groups.len() as u32;
        self.inner.p_groups = p_groups.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn max_recursion_depth(mut self, max_recursion_depth: u32) -> Self {
        self.inner.max_recursion_depth = max_recursion_depth;
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = Some(layout);
        self
    }
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: Option<vk::Pipeline>) -> Self {
        self.inner.base_pipeline_handle = base_pipeline_handle;
        self
    }
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.inner.base_pipeline_index = base_pipeline_index;
        self
    }
}
impl<'a> Deref for RayTracingPipelineCreateInfoNVBuilder<'a> {
    type Target = vk::RayTracingPipelineCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RayTracingPipelineCreateInfoKHR {
    type Type = RayTracingPipelineCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait RayTracingPipelineCreateInfoKHRNext {}
#[derive(Default)]
pub struct RayTracingPipelineCreateInfoKHRBuilder<'a> {
    inner: vk::RayTracingPipelineCreateInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RayTracingPipelineCreateInfoKHRBuilder<'a> {
    pub fn insert_next<T: RayTracingPipelineCreateInfoKHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_stages(mut self, p_stages: &'a [vk::PipelineShaderStageCreateInfo]) -> Self {
        self.inner.stage_count = p_stages.len() as u32;
        self.inner.p_stages = p_stages.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_groups(mut self, p_groups: &'a [vk::RayTracingShaderGroupCreateInfoKHR]) -> Self {
        self.inner.group_count = p_groups.len() as u32;
        self.inner.p_groups = p_groups.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn max_pipeline_ray_recursion_depth(mut self, max_pipeline_ray_recursion_depth: u32) -> Self {
        self.inner.max_pipeline_ray_recursion_depth = max_pipeline_ray_recursion_depth;
        self
    }
    pub fn p_library_info(mut self, p_library_info: Option<&'a vk::PipelineLibraryCreateInfoKHR>) -> Self {
        self.inner.p_library_info = p_library_info.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_library_interface(
        mut self,
        p_library_interface: Option<&'a vk::RayTracingPipelineInterfaceCreateInfoKHR>,
    ) -> Self {
        self.inner.p_library_interface = p_library_interface.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_dynamic_state(mut self, p_dynamic_state: Option<&'a vk::PipelineDynamicStateCreateInfo>) -> Self {
        self.inner.p_dynamic_state = p_dynamic_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = Some(layout);
        self
    }
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: Option<vk::Pipeline>) -> Self {
        self.inner.base_pipeline_handle = base_pipeline_handle;
        self
    }
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.inner.base_pipeline_index = base_pipeline_index;
        self
    }
}
impl<'a> Deref for RayTracingPipelineCreateInfoKHRBuilder<'a> {
    type Target = vk::RayTracingPipelineCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::GeometryTrianglesNV {
    type Type = GeometryTrianglesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GeometryTrianglesNVBuilder {
    inner: vk::GeometryTrianglesNV,
}
impl GeometryTrianglesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn vertex_data(mut self, vertex_data: Option<vk::Buffer>) -> Self {
        self.inner.vertex_data = vertex_data;
        self
    }
    pub fn vertex_offset(mut self, vertex_offset: vk::DeviceSize) -> Self {
        self.inner.vertex_offset = vertex_offset;
        self
    }
    pub fn vertex_count(mut self, vertex_count: u32) -> Self {
        self.inner.vertex_count = vertex_count;
        self
    }
    pub fn vertex_stride(mut self, vertex_stride: vk::DeviceSize) -> Self {
        self.inner.vertex_stride = vertex_stride;
        self
    }
    pub fn vertex_format(mut self, vertex_format: vk::Format) -> Self {
        self.inner.vertex_format = vertex_format;
        self
    }
    pub fn index_data(mut self, index_data: Option<vk::Buffer>) -> Self {
        self.inner.index_data = index_data;
        self
    }
    pub fn index_offset(mut self, index_offset: vk::DeviceSize) -> Self {
        self.inner.index_offset = index_offset;
        self
    }
    pub fn index_count(mut self, index_count: u32) -> Self {
        self.inner.index_count = index_count;
        self
    }
    pub fn index_type(mut self, index_type: vk::IndexType) -> Self {
        self.inner.index_type = index_type;
        self
    }
    pub fn transform_data(mut self, transform_data: Option<vk::Buffer>) -> Self {
        self.inner.transform_data = transform_data;
        self
    }
    pub fn transform_offset(mut self, transform_offset: vk::DeviceSize) -> Self {
        self.inner.transform_offset = transform_offset;
        self
    }
}
impl Deref for GeometryTrianglesNVBuilder {
    type Target = vk::GeometryTrianglesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::GeometryAABBNV {
    type Type = GeometryAABBNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GeometryAABBNVBuilder {
    inner: vk::GeometryAABBNV,
}
impl GeometryAABBNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn aabb_data(mut self, aabb_data: Option<vk::Buffer>) -> Self {
        self.inner.aabb_data = aabb_data;
        self
    }
    pub fn num_aab_bs(mut self, num_aab_bs: u32) -> Self {
        self.inner.num_aab_bs = num_aab_bs;
        self
    }
    pub fn stride(mut self, stride: u32) -> Self {
        self.inner.stride = stride;
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
}
impl Deref for GeometryAABBNVBuilder {
    type Target = vk::GeometryAABBNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::GeometryNV {
    type Type = GeometryNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GeometryNVBuilder {
    inner: vk::GeometryNV,
}
impl GeometryNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn geometry_type(mut self, geometry_type: vk::GeometryTypeKHR) -> Self {
        self.inner.geometry_type = geometry_type;
        self
    }
    pub fn geometry(mut self, geometry: vk::GeometryDataNV) -> Self {
        self.inner.geometry = geometry;
        self
    }
    pub fn flags(mut self, flags: vk::GeometryFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for GeometryNVBuilder {
    type Target = vk::GeometryNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AccelerationStructureInfoNV {
    type Type = AccelerationStructureInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureInfoNVBuilder<'a> {
    inner: vk::AccelerationStructureInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> AccelerationStructureInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::AccelerationStructureTypeNV) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn flags(mut self, flags: vk::BuildAccelerationStructureFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn instance_count(mut self, instance_count: u32) -> Self {
        self.inner.instance_count = instance_count;
        self
    }
    pub fn p_geometries(mut self, p_geometries: &'a [vk::GeometryNV]) -> Self {
        self.inner.geometry_count = p_geometries.len() as u32;
        self.inner.p_geometries = p_geometries.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for AccelerationStructureInfoNVBuilder<'a> {
    type Target = vk::AccelerationStructureInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AccelerationStructureCreateInfoNV {
    type Type = AccelerationStructureCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureCreateInfoNVBuilder {
    inner: vk::AccelerationStructureCreateInfoNV,
}
impl AccelerationStructureCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn compacted_size(mut self, compacted_size: vk::DeviceSize) -> Self {
        self.inner.compacted_size = compacted_size;
        self
    }
    pub fn info(mut self, info: vk::AccelerationStructureInfoNV) -> Self {
        self.inner.info = info;
        self
    }
}
impl Deref for AccelerationStructureCreateInfoNVBuilder {
    type Target = vk::AccelerationStructureCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BindAccelerationStructureMemoryInfoNV {
    type Type = BindAccelerationStructureMemoryInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindAccelerationStructureMemoryInfoNVBuilder<'a> {
    inner: vk::BindAccelerationStructureMemoryInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BindAccelerationStructureMemoryInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn acceleration_structure(mut self, acceleration_structure: vk::AccelerationStructureNV) -> Self {
        self.inner.acceleration_structure = Some(acceleration_structure);
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn memory_offset(mut self, memory_offset: vk::DeviceSize) -> Self {
        self.inner.memory_offset = memory_offset;
        self
    }
    pub fn p_device_indices(mut self, p_device_indices: &'a [u32]) -> Self {
        self.inner.device_index_count = p_device_indices.len() as u32;
        self.inner.p_device_indices = p_device_indices.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for BindAccelerationStructureMemoryInfoNVBuilder<'a> {
    type Target = vk::BindAccelerationStructureMemoryInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::WriteDescriptorSetAccelerationStructureKHR {
    type Type = WriteDescriptorSetAccelerationStructureKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    inner: vk::WriteDescriptorSetAccelerationStructureKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_acceleration_structures(mut self, p_acceleration_structures: &'a [vk::AccelerationStructureKHR]) -> Self {
        self.inner.acceleration_structure_count = p_acceleration_structures.len() as u32;
        self.inner.p_acceleration_structures = p_acceleration_structures.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    type Target = vk::WriteDescriptorSetAccelerationStructureKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> WriteDescriptorSetNext for WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {}
impl WriteDescriptorSetNext for vk::WriteDescriptorSetAccelerationStructureKHR {}
impl<'a> Builder<'a> for vk::WriteDescriptorSetAccelerationStructureNV {
    type Type = WriteDescriptorSetAccelerationStructureNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
    inner: vk::WriteDescriptorSetAccelerationStructureNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_acceleration_structures(mut self, p_acceleration_structures: &'a [vk::AccelerationStructureNV]) -> Self {
        self.inner.acceleration_structure_count = p_acceleration_structures.len() as u32;
        self.inner.p_acceleration_structures = p_acceleration_structures.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
    type Target = vk::WriteDescriptorSetAccelerationStructureNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> WriteDescriptorSetNext for WriteDescriptorSetAccelerationStructureNVBuilder<'a> {}
impl WriteDescriptorSetNext for vk::WriteDescriptorSetAccelerationStructureNV {}
impl Builder<'_> for vk::AccelerationStructureMemoryRequirementsInfoNV {
    type Type = AccelerationStructureMemoryRequirementsInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureMemoryRequirementsInfoNVBuilder {
    inner: vk::AccelerationStructureMemoryRequirementsInfoNV,
}
impl AccelerationStructureMemoryRequirementsInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::AccelerationStructureMemoryRequirementsTypeNV) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn acceleration_structure(mut self, acceleration_structure: vk::AccelerationStructureNV) -> Self {
        self.inner.acceleration_structure = Some(acceleration_structure);
        self
    }
}
impl Deref for AccelerationStructureMemoryRequirementsInfoNVBuilder {
    type Target = vk::AccelerationStructureMemoryRequirementsInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceAccelerationStructureFeaturesKHR {
    type Type = PhysicalDeviceAccelerationStructureFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceAccelerationStructureFeaturesKHRBuilder {
    inner: vk::PhysicalDeviceAccelerationStructureFeaturesKHR,
}
impl PhysicalDeviceAccelerationStructureFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn acceleration_structure(mut self, acceleration_structure: bool) -> Self {
        self.inner.acceleration_structure = if acceleration_structure { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn acceleration_structure_capture_replay(mut self, acceleration_structure_capture_replay: bool) -> Self {
        self.inner.acceleration_structure_capture_replay = if acceleration_structure_capture_replay {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn acceleration_structure_indirect_build(mut self, acceleration_structure_indirect_build: bool) -> Self {
        self.inner.acceleration_structure_indirect_build = if acceleration_structure_indirect_build {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn acceleration_structure_host_commands(mut self, acceleration_structure_host_commands: bool) -> Self {
        self.inner.acceleration_structure_host_commands = if acceleration_structure_host_commands {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn descriptor_binding_acceleration_structure_update_after_bind(
        mut self,
        descriptor_binding_acceleration_structure_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_acceleration_structure_update_after_bind =
            if descriptor_binding_acceleration_structure_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
}
impl Deref for PhysicalDeviceAccelerationStructureFeaturesKHRBuilder {
    type Target = vk::PhysicalDeviceAccelerationStructureFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceAccelerationStructureFeaturesKHRBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceAccelerationStructureFeaturesKHRBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceAccelerationStructureFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceAccelerationStructureFeaturesKHR {}
impl Builder<'_> for vk::PhysicalDeviceRayTracingPipelineFeaturesKHR {
    type Type = PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder {
    inner: vk::PhysicalDeviceRayTracingPipelineFeaturesKHR,
}
impl PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ray_tracing_pipeline(mut self, ray_tracing_pipeline: bool) -> Self {
        self.inner.ray_tracing_pipeline = if ray_tracing_pipeline { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn ray_tracing_pipeline_shader_group_handle_capture_replay(
        mut self,
        ray_tracing_pipeline_shader_group_handle_capture_replay: bool,
    ) -> Self {
        self.inner.ray_tracing_pipeline_shader_group_handle_capture_replay =
            if ray_tracing_pipeline_shader_group_handle_capture_replay {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn ray_tracing_pipeline_shader_group_handle_capture_replay_mixed(
        mut self,
        ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: bool,
    ) -> Self {
        self.inner.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed =
            if ray_tracing_pipeline_shader_group_handle_capture_replay_mixed {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn ray_tracing_pipeline_trace_rays_indirect(mut self, ray_tracing_pipeline_trace_rays_indirect: bool) -> Self {
        self.inner.ray_tracing_pipeline_trace_rays_indirect = if ray_tracing_pipeline_trace_rays_indirect {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn ray_traversal_primitive_culling(mut self, ray_traversal_primitive_culling: bool) -> Self {
        self.inner.ray_traversal_primitive_culling = if ray_traversal_primitive_culling {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder {
    type Target = vk::PhysicalDeviceRayTracingPipelineFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRayTracingPipelineFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRayTracingPipelineFeaturesKHR {}
impl Builder<'_> for vk::PhysicalDeviceRayQueryFeaturesKHR {
    type Type = PhysicalDeviceRayQueryFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceRayQueryFeaturesKHRBuilder {
    inner: vk::PhysicalDeviceRayQueryFeaturesKHR,
}
impl PhysicalDeviceRayQueryFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ray_query(mut self, ray_query: bool) -> Self {
        self.inner.ray_query = if ray_query { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceRayQueryFeaturesKHRBuilder {
    type Target = vk::PhysicalDeviceRayQueryFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceRayQueryFeaturesKHRBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceRayQueryFeaturesKHRBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRayQueryFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRayQueryFeaturesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceAccelerationStructurePropertiesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceRayTracingPipelinePropertiesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceRayTracingPropertiesNV {}
impl FormatProperties2Next for vk::DrmFormatModifierPropertiesListEXT {}
impl<'a> Builder<'a> for vk::PhysicalDeviceImageDrmFormatModifierInfoEXT {
    type Type = PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    inner: vk::PhysicalDeviceImageDrmFormatModifierInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.inner.drm_format_modifier = drm_format_modifier;
        self
    }
    pub fn sharing_mode(mut self, sharing_mode: vk::SharingMode) -> Self {
        self.inner.sharing_mode = sharing_mode;
        self
    }
    pub fn p_queue_family_indices(mut self, p_queue_family_indices: &'a [u32]) -> Self {
        self.inner.queue_family_index_count = p_queue_family_indices.len() as u32;
        self.inner.p_queue_family_indices = p_queue_family_indices.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceImageDrmFormatModifierInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> PhysicalDeviceImageFormatInfo2Next for PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {}
impl PhysicalDeviceImageFormatInfo2Next for vk::PhysicalDeviceImageDrmFormatModifierInfoEXT {}
impl<'a> Builder<'a> for vk::ImageDrmFormatModifierListCreateInfoEXT {
    type Type = ImageDrmFormatModifierListCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    inner: vk::ImageDrmFormatModifierListCreateInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_drm_format_modifiers(mut self, p_drm_format_modifiers: &'a [u64]) -> Self {
        self.inner.drm_format_modifier_count = p_drm_format_modifiers.len() as u32;
        self.inner.p_drm_format_modifiers = p_drm_format_modifiers.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    type Target = vk::ImageDrmFormatModifierListCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> ImageCreateInfoNext for ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {}
impl ImageCreateInfoNext for vk::ImageDrmFormatModifierListCreateInfoEXT {}
impl<'a> Builder<'a> for vk::ImageDrmFormatModifierExplicitCreateInfoEXT {
    type Type = ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    inner: vk::ImageDrmFormatModifierExplicitCreateInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.inner.drm_format_modifier = drm_format_modifier;
        self
    }
    pub fn p_plane_layouts(mut self, p_plane_layouts: &'a [vk::SubresourceLayout]) -> Self {
        self.inner.drm_format_modifier_plane_count = p_plane_layouts.len() as u32;
        self.inner.p_plane_layouts = p_plane_layouts.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    type Target = vk::ImageDrmFormatModifierExplicitCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> ImageCreateInfoNext for ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {}
impl ImageCreateInfoNext for vk::ImageDrmFormatModifierExplicitCreateInfoEXT {}
impl Builder<'_> for vk::ImageStencilUsageCreateInfo {
    type Type = ImageStencilUsageCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageStencilUsageCreateInfoBuilder {
    inner: vk::ImageStencilUsageCreateInfo,
}
impl ImageStencilUsageCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn stencil_usage(mut self, stencil_usage: vk::ImageUsageFlags) -> Self {
        self.inner.stencil_usage = stencil_usage;
        self
    }
}
impl Deref for ImageStencilUsageCreateInfoBuilder {
    type Target = vk::ImageStencilUsageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageCreateInfoNext for ImageStencilUsageCreateInfoBuilder {}
impl PhysicalDeviceImageFormatInfo2Next for ImageStencilUsageCreateInfoBuilder {}
impl ImageCreateInfoNext for vk::ImageStencilUsageCreateInfo {}
impl PhysicalDeviceImageFormatInfo2Next for vk::ImageStencilUsageCreateInfo {}
impl Builder<'_> for vk::DeviceMemoryOverallocationCreateInfoAMD {
    type Type = DeviceMemoryOverallocationCreateInfoAMDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceMemoryOverallocationCreateInfoAMDBuilder {
    inner: vk::DeviceMemoryOverallocationCreateInfoAMD,
}
impl DeviceMemoryOverallocationCreateInfoAMDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn overallocation_behavior(mut self, overallocation_behavior: vk::MemoryOverallocationBehaviorAMD) -> Self {
        self.inner.overallocation_behavior = overallocation_behavior;
        self
    }
}
impl Deref for DeviceMemoryOverallocationCreateInfoAMDBuilder {
    type Target = vk::DeviceMemoryOverallocationCreateInfoAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DeviceCreateInfoNext for DeviceMemoryOverallocationCreateInfoAMDBuilder {}
impl DeviceCreateInfoNext for vk::DeviceMemoryOverallocationCreateInfoAMD {}
impl Builder<'_> for vk::PhysicalDeviceFragmentDensityMapFeaturesEXT {
    type Type = PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceFragmentDensityMapFeaturesEXT,
}
impl PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_density_map(mut self, fragment_density_map: bool) -> Self {
        self.inner.fragment_density_map = if fragment_density_map { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn fragment_density_map_dynamic(mut self, fragment_density_map_dynamic: bool) -> Self {
        self.inner.fragment_density_map_dynamic = if fragment_density_map_dynamic {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn fragment_density_map_non_subsampled_images(
        mut self,
        fragment_density_map_non_subsampled_images: bool,
    ) -> Self {
        self.inner.fragment_density_map_non_subsampled_images = if fragment_density_map_non_subsampled_images {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceFragmentDensityMapFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFragmentDensityMapFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFragmentDensityMapFeaturesEXT {}
impl Builder<'_> for vk::PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    type Type = PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder {
    inner: vk::PhysicalDeviceFragmentDensityMap2FeaturesEXT,
}
impl PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_density_map_deferred(mut self, fragment_density_map_deferred: bool) -> Self {
        self.inner.fragment_density_map_deferred = if fragment_density_map_deferred {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceFragmentDensityMap2FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFragmentDensityMap2FeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFragmentDensityMap2FeaturesEXT {}
impl Builder<'_> for vk::PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
    type Type = PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder {
    inner: vk::PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM,
}
impl PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_density_map_offset(mut self, fragment_density_map_offset: bool) -> Self {
        self.inner.fragment_density_map_offset = if fragment_density_map_offset {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder {
    type Target = vk::PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOMBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceFragmentDensityMapPropertiesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceFragmentDensityMap2PropertiesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {}
impl Builder<'_> for vk::RenderPassFragmentDensityMapCreateInfoEXT {
    type Type = RenderPassFragmentDensityMapCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassFragmentDensityMapCreateInfoEXTBuilder {
    inner: vk::RenderPassFragmentDensityMapCreateInfoEXT,
}
impl RenderPassFragmentDensityMapCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_density_map_attachment(mut self, fragment_density_map_attachment: vk::AttachmentReference) -> Self {
        self.inner.fragment_density_map_attachment = fragment_density_map_attachment;
        self
    }
}
impl Deref for RenderPassFragmentDensityMapCreateInfoEXTBuilder {
    type Target = vk::RenderPassFragmentDensityMapCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl RenderPassCreateInfoNext for RenderPassFragmentDensityMapCreateInfoEXTBuilder {}
impl RenderPassCreateInfo2Next for RenderPassFragmentDensityMapCreateInfoEXTBuilder {}
impl RenderPassCreateInfoNext for vk::RenderPassFragmentDensityMapCreateInfoEXT {}
impl RenderPassCreateInfo2Next for vk::RenderPassFragmentDensityMapCreateInfoEXT {}
impl<'a> Builder<'a> for vk::SubpassFragmentDensityMapOffsetEndInfoQCOM {
    type Type = SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder<'a> {
    inner: vk::SubpassFragmentDensityMapOffsetEndInfoQCOM,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_fragment_density_offsets(mut self, p_fragment_density_offsets: &'a [vk::Offset2D]) -> Self {
        self.inner.fragment_density_offset_count = p_fragment_density_offsets.len() as u32;
        self.inner.p_fragment_density_offsets = p_fragment_density_offsets
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder<'a> {
    type Target = vk::SubpassFragmentDensityMapOffsetEndInfoQCOM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> SubpassEndInfoNext for SubpassFragmentDensityMapOffsetEndInfoQCOMBuilder<'a> {}
impl SubpassEndInfoNext for vk::SubpassFragmentDensityMapOffsetEndInfoQCOM {}
impl Builder<'_> for vk::PhysicalDeviceScalarBlockLayoutFeatures {
    type Type = PhysicalDeviceScalarBlockLayoutFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceScalarBlockLayoutFeaturesBuilder {
    inner: vk::PhysicalDeviceScalarBlockLayoutFeatures,
}
impl PhysicalDeviceScalarBlockLayoutFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn scalar_block_layout(mut self, scalar_block_layout: bool) -> Self {
        self.inner.scalar_block_layout = if scalar_block_layout { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceScalarBlockLayoutFeaturesBuilder {
    type Target = vk::PhysicalDeviceScalarBlockLayoutFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceScalarBlockLayoutFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceScalarBlockLayoutFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceScalarBlockLayoutFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceScalarBlockLayoutFeatures {}
impl Builder<'_> for vk::SurfaceProtectedCapabilitiesKHR {
    type Type = SurfaceProtectedCapabilitiesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SurfaceProtectedCapabilitiesKHRBuilder {
    inner: vk::SurfaceProtectedCapabilitiesKHR,
}
impl SurfaceProtectedCapabilitiesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn supports_protected(mut self, supports_protected: bool) -> Self {
        self.inner.supports_protected = if supports_protected { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for SurfaceProtectedCapabilitiesKHRBuilder {
    type Target = vk::SurfaceProtectedCapabilitiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SurfaceCapabilities2KHRNext for SurfaceProtectedCapabilitiesKHRBuilder {}
impl SurfaceCapabilities2KHRNext for vk::SurfaceProtectedCapabilitiesKHR {}
impl Builder<'_> for vk::PhysicalDeviceUniformBufferStandardLayoutFeatures {
    type Type = PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder {
    inner: vk::PhysicalDeviceUniformBufferStandardLayoutFeatures,
}
impl PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn uniform_buffer_standard_layout(mut self, uniform_buffer_standard_layout: bool) -> Self {
        self.inner.uniform_buffer_standard_layout = if uniform_buffer_standard_layout {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder {
    type Target = vk::PhysicalDeviceUniformBufferStandardLayoutFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceUniformBufferStandardLayoutFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceUniformBufferStandardLayoutFeatures {}
impl Builder<'_> for vk::PhysicalDeviceDepthClipEnableFeaturesEXT {
    type Type = PhysicalDeviceDepthClipEnableFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceDepthClipEnableFeaturesEXT,
}
impl PhysicalDeviceDepthClipEnableFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
        self.inner.depth_clip_enable = if depth_clip_enable { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceDepthClipEnableFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceDepthClipEnableFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceDepthClipEnableFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceDepthClipEnableFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDepthClipEnableFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDepthClipEnableFeaturesEXT {}
impl Builder<'_> for vk::PipelineRasterizationDepthClipStateCreateInfoEXT {
    type Type = PipelineRasterizationDepthClipStateCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationDepthClipStateCreateInfoEXTBuilder {
    inner: vk::PipelineRasterizationDepthClipStateCreateInfoEXT,
}
impl PipelineRasterizationDepthClipStateCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineRasterizationDepthClipStateCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
        self.inner.depth_clip_enable = if depth_clip_enable { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PipelineRasterizationDepthClipStateCreateInfoEXTBuilder {
    type Target = vk::PipelineRasterizationDepthClipStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineRasterizationStateCreateInfoNext for PipelineRasterizationDepthClipStateCreateInfoEXTBuilder {}
impl PipelineRasterizationStateCreateInfoNext for vk::PipelineRasterizationDepthClipStateCreateInfoEXT {}
impl PhysicalDeviceMemoryProperties2Next for vk::PhysicalDeviceMemoryBudgetPropertiesEXT {}
impl Builder<'_> for vk::PhysicalDeviceMemoryPriorityFeaturesEXT {
    type Type = PhysicalDeviceMemoryPriorityFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceMemoryPriorityFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceMemoryPriorityFeaturesEXT,
}
impl PhysicalDeviceMemoryPriorityFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory_priority(mut self, memory_priority: bool) -> Self {
        self.inner.memory_priority = if memory_priority { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceMemoryPriorityFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceMemoryPriorityFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceMemoryPriorityFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceMemoryPriorityFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMemoryPriorityFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMemoryPriorityFeaturesEXT {}
impl Builder<'_> for vk::MemoryPriorityAllocateInfoEXT {
    type Type = MemoryPriorityAllocateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryPriorityAllocateInfoEXTBuilder {
    inner: vk::MemoryPriorityAllocateInfoEXT,
}
impl MemoryPriorityAllocateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn priority(mut self, priority: f32) -> Self {
        self.inner.priority = priority;
        self
    }
}
impl Deref for MemoryPriorityAllocateInfoEXTBuilder {
    type Target = vk::MemoryPriorityAllocateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for MemoryPriorityAllocateInfoEXTBuilder {}
impl MemoryAllocateInfoNext for vk::MemoryPriorityAllocateInfoEXT {}
impl Builder<'_> for vk::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    type Type = PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder {
    inner: vk::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT,
}
impl PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pageable_device_local_memory(mut self, pageable_device_local_memory: bool) -> Self {
        self.inner.pageable_device_local_memory = if pageable_device_local_memory {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder {
    type Target = vk::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {}
impl Builder<'_> for vk::PhysicalDeviceBufferDeviceAddressFeatures {
    type Type = PhysicalDeviceBufferDeviceAddressFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesBuilder {
    inner: vk::PhysicalDeviceBufferDeviceAddressFeatures,
}
impl PhysicalDeviceBufferDeviceAddressFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer_device_address(mut self, buffer_device_address: bool) -> Self {
        self.inner.buffer_device_address = if buffer_device_address { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn buffer_device_address_capture_replay(mut self, buffer_device_address_capture_replay: bool) -> Self {
        self.inner.buffer_device_address_capture_replay = if buffer_device_address_capture_replay {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn buffer_device_address_multi_device(mut self, buffer_device_address_multi_device: bool) -> Self {
        self.inner.buffer_device_address_multi_device = if buffer_device_address_multi_device {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceBufferDeviceAddressFeaturesBuilder {
    type Target = vk::PhysicalDeviceBufferDeviceAddressFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceBufferDeviceAddressFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceBufferDeviceAddressFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceBufferDeviceAddressFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceBufferDeviceAddressFeatures {}
impl Builder<'_> for vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    type Type = PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT,
}
impl PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer_device_address(mut self, buffer_device_address: bool) -> Self {
        self.inner.buffer_device_address = if buffer_device_address { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn buffer_device_address_capture_replay(mut self, buffer_device_address_capture_replay: bool) -> Self {
        self.inner.buffer_device_address_capture_replay = if buffer_device_address_capture_replay {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn buffer_device_address_multi_device(mut self, buffer_device_address_multi_device: bool) -> Self {
        self.inner.buffer_device_address_multi_device = if buffer_device_address_multi_device {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT {}
impl Builder<'_> for vk::BufferDeviceAddressInfo {
    type Type = BufferDeviceAddressInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferDeviceAddressInfoBuilder {
    inner: vk::BufferDeviceAddressInfo,
}
impl BufferDeviceAddressInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
}
impl Deref for BufferDeviceAddressInfoBuilder {
    type Target = vk::BufferDeviceAddressInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BufferOpaqueCaptureAddressCreateInfo {
    type Type = BufferOpaqueCaptureAddressCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferOpaqueCaptureAddressCreateInfoBuilder {
    inner: vk::BufferOpaqueCaptureAddressCreateInfo,
}
impl BufferOpaqueCaptureAddressCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn opaque_capture_address(mut self, opaque_capture_address: u64) -> Self {
        self.inner.opaque_capture_address = opaque_capture_address;
        self
    }
}
impl Deref for BufferOpaqueCaptureAddressCreateInfoBuilder {
    type Target = vk::BufferOpaqueCaptureAddressCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl BufferCreateInfoNext for BufferOpaqueCaptureAddressCreateInfoBuilder {}
impl BufferCreateInfoNext for vk::BufferOpaqueCaptureAddressCreateInfo {}
impl Builder<'_> for vk::BufferDeviceAddressCreateInfoEXT {
    type Type = BufferDeviceAddressCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferDeviceAddressCreateInfoEXTBuilder {
    inner: vk::BufferDeviceAddressCreateInfoEXT,
}
impl BufferDeviceAddressCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_address(mut self, device_address: vk::DeviceAddress) -> Self {
        self.inner.device_address = device_address;
        self
    }
}
impl Deref for BufferDeviceAddressCreateInfoEXTBuilder {
    type Target = vk::BufferDeviceAddressCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl BufferCreateInfoNext for BufferDeviceAddressCreateInfoEXTBuilder {}
impl BufferCreateInfoNext for vk::BufferDeviceAddressCreateInfoEXT {}
impl Builder<'_> for vk::PhysicalDeviceImageViewImageFormatInfoEXT {
    type Type = PhysicalDeviceImageViewImageFormatInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXTBuilder {
    inner: vk::PhysicalDeviceImageViewImageFormatInfoEXT,
}
impl PhysicalDeviceImageViewImageFormatInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image_view_type(mut self, image_view_type: vk::ImageViewType) -> Self {
        self.inner.image_view_type = image_view_type;
        self
    }
}
impl Deref for PhysicalDeviceImageViewImageFormatInfoEXTBuilder {
    type Target = vk::PhysicalDeviceImageViewImageFormatInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceImageFormatInfo2Next for PhysicalDeviceImageViewImageFormatInfoEXTBuilder {}
impl PhysicalDeviceImageFormatInfo2Next for vk::PhysicalDeviceImageViewImageFormatInfoEXT {}
impl ImageFormatProperties2Next for vk::FilterCubicImageViewImageFormatPropertiesEXT {}
impl Builder<'_> for vk::PhysicalDeviceImagelessFramebufferFeatures {
    type Type = PhysicalDeviceImagelessFramebufferFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceImagelessFramebufferFeaturesBuilder {
    inner: vk::PhysicalDeviceImagelessFramebufferFeatures,
}
impl PhysicalDeviceImagelessFramebufferFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn imageless_framebuffer(mut self, imageless_framebuffer: bool) -> Self {
        self.inner.imageless_framebuffer = if imageless_framebuffer { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceImagelessFramebufferFeaturesBuilder {
    type Target = vk::PhysicalDeviceImagelessFramebufferFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceImagelessFramebufferFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceImagelessFramebufferFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceImagelessFramebufferFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceImagelessFramebufferFeatures {}
impl<'a> Builder<'a> for vk::FramebufferAttachmentsCreateInfo {
    type Type = FramebufferAttachmentsCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FramebufferAttachmentsCreateInfoBuilder<'a> {
    inner: vk::FramebufferAttachmentsCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> FramebufferAttachmentsCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attachment_image_infos(
        mut self,
        p_attachment_image_infos: &'a [vk::FramebufferAttachmentImageInfo],
    ) -> Self {
        self.inner.attachment_image_info_count = p_attachment_image_infos.len() as u32;
        self.inner.p_attachment_image_infos = p_attachment_image_infos.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for FramebufferAttachmentsCreateInfoBuilder<'a> {
    type Target = vk::FramebufferAttachmentsCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> FramebufferCreateInfoNext for FramebufferAttachmentsCreateInfoBuilder<'a> {}
impl FramebufferCreateInfoNext for vk::FramebufferAttachmentsCreateInfo {}
impl<'a> Builder<'a> for vk::FramebufferAttachmentImageInfo {
    type Type = FramebufferAttachmentImageInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FramebufferAttachmentImageInfoBuilder<'a> {
    inner: vk::FramebufferAttachmentImageInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> FramebufferAttachmentImageInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ImageCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn width(mut self, width: u32) -> Self {
        self.inner.width = width;
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.inner.height = height;
        self
    }
    pub fn layer_count(mut self, layer_count: u32) -> Self {
        self.inner.layer_count = layer_count;
        self
    }
    pub fn p_view_formats(mut self, p_view_formats: &'a [vk::Format]) -> Self {
        self.inner.view_format_count = p_view_formats.len() as u32;
        self.inner.p_view_formats = p_view_formats.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for FramebufferAttachmentImageInfoBuilder<'a> {
    type Target = vk::FramebufferAttachmentImageInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RenderPassAttachmentBeginInfo {
    type Type = RenderPassAttachmentBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassAttachmentBeginInfoBuilder<'a> {
    inner: vk::RenderPassAttachmentBeginInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RenderPassAttachmentBeginInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attachments(mut self, p_attachments: &'a [vk::ImageView]) -> Self {
        self.inner.attachment_count = p_attachments.len() as u32;
        self.inner.p_attachments = p_attachments.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for RenderPassAttachmentBeginInfoBuilder<'a> {
    type Target = vk::RenderPassAttachmentBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> RenderPassBeginInfoNext for RenderPassAttachmentBeginInfoBuilder<'a> {}
impl RenderPassBeginInfoNext for vk::RenderPassAttachmentBeginInfo {}
impl Builder<'_> for vk::PhysicalDeviceTextureCompressionASTCHDRFeatures {
    type Type = PhysicalDeviceTextureCompressionASTCHDRFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceTextureCompressionASTCHDRFeaturesBuilder {
    inner: vk::PhysicalDeviceTextureCompressionASTCHDRFeatures,
}
impl PhysicalDeviceTextureCompressionASTCHDRFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn texture_compression_astc_hdr(mut self, texture_compression_astc_hdr: bool) -> Self {
        self.inner.texture_compression_astc_hdr = if texture_compression_astc_hdr {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceTextureCompressionASTCHDRFeaturesBuilder {
    type Target = vk::PhysicalDeviceTextureCompressionASTCHDRFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceTextureCompressionASTCHDRFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceTextureCompressionASTCHDRFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceTextureCompressionASTCHDRFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceTextureCompressionASTCHDRFeatures {}
impl Builder<'_> for vk::PhysicalDeviceCooperativeMatrixFeaturesNV {
    type Type = PhysicalDeviceCooperativeMatrixFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNVBuilder {
    inner: vk::PhysicalDeviceCooperativeMatrixFeaturesNV,
}
impl PhysicalDeviceCooperativeMatrixFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn cooperative_matrix(mut self, cooperative_matrix: bool) -> Self {
        self.inner.cooperative_matrix = if cooperative_matrix { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn cooperative_matrix_robust_buffer_access(mut self, cooperative_matrix_robust_buffer_access: bool) -> Self {
        self.inner.cooperative_matrix_robust_buffer_access = if cooperative_matrix_robust_buffer_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceCooperativeMatrixFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceCooperativeMatrixFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceCooperativeMatrixFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceCooperativeMatrixFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCooperativeMatrixFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCooperativeMatrixFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceCooperativeMatrixPropertiesNV {}
impl Builder<'_> for vk::CooperativeMatrixPropertiesNV {
    type Type = CooperativeMatrixPropertiesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CooperativeMatrixPropertiesNVBuilder {
    inner: vk::CooperativeMatrixPropertiesNV,
}
impl CooperativeMatrixPropertiesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn m_size(mut self, m_size: u32) -> Self {
        self.inner.m_size = m_size;
        self
    }
    pub fn n_size(mut self, n_size: u32) -> Self {
        self.inner.n_size = n_size;
        self
    }
    pub fn k_size(mut self, k_size: u32) -> Self {
        self.inner.k_size = k_size;
        self
    }
    pub fn a_type(mut self, a_type: vk::ComponentTypeNV) -> Self {
        self.inner.a_type = a_type;
        self
    }
    pub fn b_type(mut self, b_type: vk::ComponentTypeNV) -> Self {
        self.inner.b_type = b_type;
        self
    }
    pub fn c_type(mut self, c_type: vk::ComponentTypeNV) -> Self {
        self.inner.c_type = c_type;
        self
    }
    pub fn d_type(mut self, d_type: vk::ComponentTypeNV) -> Self {
        self.inner.d_type = d_type;
        self
    }
    pub fn scope(mut self, scope: vk::ScopeNV) -> Self {
        self.inner.scope = scope;
        self
    }
}
impl Deref for CooperativeMatrixPropertiesNVBuilder {
    type Target = vk::CooperativeMatrixPropertiesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    type Type = PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT,
}
impl PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ycbcr_image_arrays(mut self, ycbcr_image_arrays: bool) -> Self {
        self.inner.ycbcr_image_arrays = if ycbcr_image_arrays { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT {}
impl Builder<'_> for vk::ImageViewHandleInfoNVX {
    type Type = ImageViewHandleInfoNVXBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageViewHandleInfoNVXBuilder {
    inner: vk::ImageViewHandleInfoNVX,
}
impl ImageViewHandleInfoNVXBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image_view(mut self, image_view: vk::ImageView) -> Self {
        self.inner.image_view = Some(image_view);
        self
    }
    pub fn descriptor_type(mut self, descriptor_type: vk::DescriptorType) -> Self {
        self.inner.descriptor_type = descriptor_type;
        self
    }
    pub fn sampler(mut self, sampler: Option<vk::Sampler>) -> Self {
        self.inner.sampler = sampler;
        self
    }
}
impl Deref for ImageViewHandleInfoNVXBuilder {
    type Target = vk::ImageViewHandleInfoNVX;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineCreationFeedbackCreateInfo {
    type Type = PipelineCreationFeedbackCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCreationFeedbackCreateInfoBuilder {
    inner: vk::PipelineCreationFeedbackCreateInfo,
}
impl PipelineCreationFeedbackCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_pipeline_creation_feedback(
        mut self,
        p_pipeline_creation_feedback: *mut vk::PipelineCreationFeedback,
    ) -> Self {
        self.inner.p_pipeline_creation_feedback = p_pipeline_creation_feedback;
        self
    }
    pub fn pipeline_stage_creation_feedback_count(mut self, pipeline_stage_creation_feedback_count: u32) -> Self {
        self.inner.pipeline_stage_creation_feedback_count = pipeline_stage_creation_feedback_count;
        self
    }
    pub fn p_pipeline_stage_creation_feedbacks(
        mut self,
        p_pipeline_stage_creation_feedbacks: *mut vk::PipelineCreationFeedback,
    ) -> Self {
        self.inner.p_pipeline_stage_creation_feedbacks = p_pipeline_stage_creation_feedbacks;
        self
    }
}
impl Deref for PipelineCreationFeedbackCreateInfoBuilder {
    type Target = vk::PipelineCreationFeedbackCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl GraphicsPipelineCreateInfoNext for PipelineCreationFeedbackCreateInfoBuilder {}
impl ComputePipelineCreateInfoNext for PipelineCreationFeedbackCreateInfoBuilder {}
impl RayTracingPipelineCreateInfoNVNext for PipelineCreationFeedbackCreateInfoBuilder {}
impl RayTracingPipelineCreateInfoKHRNext for PipelineCreationFeedbackCreateInfoBuilder {}
impl GraphicsPipelineCreateInfoNext for vk::PipelineCreationFeedbackCreateInfo {}
impl ComputePipelineCreateInfoNext for vk::PipelineCreationFeedbackCreateInfo {}
impl RayTracingPipelineCreateInfoNVNext for vk::PipelineCreationFeedbackCreateInfo {}
impl RayTracingPipelineCreateInfoKHRNext for vk::PipelineCreationFeedbackCreateInfo {}
impl Builder<'_> for vk::SurfaceFullScreenExclusiveInfoEXT {
    type Type = SurfaceFullScreenExclusiveInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SurfaceFullScreenExclusiveInfoEXTBuilder {
    inner: vk::SurfaceFullScreenExclusiveInfoEXT,
}
impl SurfaceFullScreenExclusiveInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn full_screen_exclusive(mut self, full_screen_exclusive: vk::FullScreenExclusiveEXT) -> Self {
        self.inner.full_screen_exclusive = full_screen_exclusive;
        self
    }
}
impl Deref for SurfaceFullScreenExclusiveInfoEXTBuilder {
    type Target = vk::SurfaceFullScreenExclusiveInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceSurfaceInfo2KHRNext for SurfaceFullScreenExclusiveInfoEXTBuilder {}
impl SwapchainCreateInfoKHRNext for SurfaceFullScreenExclusiveInfoEXTBuilder {}
impl PhysicalDeviceSurfaceInfo2KHRNext for vk::SurfaceFullScreenExclusiveInfoEXT {}
impl SwapchainCreateInfoKHRNext for vk::SurfaceFullScreenExclusiveInfoEXT {}
impl Builder<'_> for vk::SurfaceFullScreenExclusiveWin32InfoEXT {
    type Type = SurfaceFullScreenExclusiveWin32InfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXTBuilder {
    inner: vk::SurfaceFullScreenExclusiveWin32InfoEXT,
}
impl SurfaceFullScreenExclusiveWin32InfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn hmonitor(mut self, hmonitor: vk::HMONITOR) -> Self {
        self.inner.hmonitor = hmonitor;
        self
    }
}
impl Deref for SurfaceFullScreenExclusiveWin32InfoEXTBuilder {
    type Target = vk::SurfaceFullScreenExclusiveWin32InfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceSurfaceInfo2KHRNext for SurfaceFullScreenExclusiveWin32InfoEXTBuilder {}
impl SwapchainCreateInfoKHRNext for SurfaceFullScreenExclusiveWin32InfoEXTBuilder {}
impl PhysicalDeviceSurfaceInfo2KHRNext for vk::SurfaceFullScreenExclusiveWin32InfoEXT {}
impl SwapchainCreateInfoKHRNext for vk::SurfaceFullScreenExclusiveWin32InfoEXT {}
impl Builder<'_> for vk::SurfaceCapabilitiesFullScreenExclusiveEXT {
    type Type = SurfaceCapabilitiesFullScreenExclusiveEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXTBuilder {
    inner: vk::SurfaceCapabilitiesFullScreenExclusiveEXT,
}
impl SurfaceCapabilitiesFullScreenExclusiveEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn full_screen_exclusive_supported(mut self, full_screen_exclusive_supported: bool) -> Self {
        self.inner.full_screen_exclusive_supported = if full_screen_exclusive_supported {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for SurfaceCapabilitiesFullScreenExclusiveEXTBuilder {
    type Target = vk::SurfaceCapabilitiesFullScreenExclusiveEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SurfaceCapabilities2KHRNext for SurfaceCapabilitiesFullScreenExclusiveEXTBuilder {}
impl SurfaceCapabilities2KHRNext for vk::SurfaceCapabilitiesFullScreenExclusiveEXT {}
impl Builder<'_> for vk::PhysicalDevicePerformanceQueryFeaturesKHR {
    type Type = PhysicalDevicePerformanceQueryFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePerformanceQueryFeaturesKHRBuilder {
    inner: vk::PhysicalDevicePerformanceQueryFeaturesKHR,
}
impl PhysicalDevicePerformanceQueryFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn performance_counter_query_pools(mut self, performance_counter_query_pools: bool) -> Self {
        self.inner.performance_counter_query_pools = if performance_counter_query_pools {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn performance_counter_multiple_query_pools(mut self, performance_counter_multiple_query_pools: bool) -> Self {
        self.inner.performance_counter_multiple_query_pools = if performance_counter_multiple_query_pools {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDevicePerformanceQueryFeaturesKHRBuilder {
    type Target = vk::PhysicalDevicePerformanceQueryFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDevicePerformanceQueryFeaturesKHRBuilder {}
impl DeviceCreateInfoNext for PhysicalDevicePerformanceQueryFeaturesKHRBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePerformanceQueryFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePerformanceQueryFeaturesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDevicePerformanceQueryPropertiesKHR {}
impl<'a> Builder<'a> for vk::QueryPoolPerformanceCreateInfoKHR {
    type Type = QueryPoolPerformanceCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    inner: vk::QueryPoolPerformanceCreateInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.inner.queue_family_index = queue_family_index;
        self
    }
    pub fn p_counter_indices(mut self, p_counter_indices: &'a [u32]) -> Self {
        self.inner.counter_index_count = p_counter_indices.len() as u32;
        self.inner.p_counter_indices = p_counter_indices.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    type Target = vk::QueryPoolPerformanceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> QueryPoolCreateInfoNext for QueryPoolPerformanceCreateInfoKHRBuilder<'a> {}
impl QueryPoolCreateInfoNext for vk::QueryPoolPerformanceCreateInfoKHR {}
impl Builder<'_> for vk::AcquireProfilingLockInfoKHR {
    type Type = AcquireProfilingLockInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AcquireProfilingLockInfoKHRBuilder {
    inner: vk::AcquireProfilingLockInfoKHR,
}
impl AcquireProfilingLockInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::AcquireProfilingLockFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.inner.timeout = timeout;
        self
    }
}
impl Deref for AcquireProfilingLockInfoKHRBuilder {
    type Target = vk::AcquireProfilingLockInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PerformanceQuerySubmitInfoKHR {
    type Type = PerformanceQuerySubmitInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PerformanceQuerySubmitInfoKHRBuilder {
    inner: vk::PerformanceQuerySubmitInfoKHR,
}
impl PerformanceQuerySubmitInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn counter_pass_index(mut self, counter_pass_index: u32) -> Self {
        self.inner.counter_pass_index = counter_pass_index;
        self
    }
}
impl Deref for PerformanceQuerySubmitInfoKHRBuilder {
    type Target = vk::PerformanceQuerySubmitInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubmitInfoNext for PerformanceQuerySubmitInfoKHRBuilder {}
impl SubmitInfo2Next for PerformanceQuerySubmitInfoKHRBuilder {}
impl SubmitInfoNext for vk::PerformanceQuerySubmitInfoKHR {}
impl SubmitInfo2Next for vk::PerformanceQuerySubmitInfoKHR {}
impl Builder<'_> for vk::HeadlessSurfaceCreateInfoEXT {
    type Type = HeadlessSurfaceCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct HeadlessSurfaceCreateInfoEXTBuilder {
    inner: vk::HeadlessSurfaceCreateInfoEXT,
}
impl HeadlessSurfaceCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::HeadlessSurfaceCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for HeadlessSurfaceCreateInfoEXTBuilder {
    type Target = vk::HeadlessSurfaceCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceCoverageReductionModeFeaturesNV {
    type Type = PhysicalDeviceCoverageReductionModeFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNVBuilder {
    inner: vk::PhysicalDeviceCoverageReductionModeFeaturesNV,
}
impl PhysicalDeviceCoverageReductionModeFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn coverage_reduction_mode(mut self, coverage_reduction_mode: bool) -> Self {
        self.inner.coverage_reduction_mode = if coverage_reduction_mode { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceCoverageReductionModeFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceCoverageReductionModeFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceCoverageReductionModeFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceCoverageReductionModeFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCoverageReductionModeFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCoverageReductionModeFeaturesNV {}
impl Builder<'_> for vk::PipelineCoverageReductionStateCreateInfoNV {
    type Type = PipelineCoverageReductionStateCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCoverageReductionStateCreateInfoNVBuilder {
    inner: vk::PipelineCoverageReductionStateCreateInfoNV,
}
impl PipelineCoverageReductionStateCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCoverageReductionStateCreateFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn coverage_reduction_mode(mut self, coverage_reduction_mode: vk::CoverageReductionModeNV) -> Self {
        self.inner.coverage_reduction_mode = coverage_reduction_mode;
        self
    }
}
impl Deref for PipelineCoverageReductionStateCreateInfoNVBuilder {
    type Target = vk::PipelineCoverageReductionStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineMultisampleStateCreateInfoNext for PipelineCoverageReductionStateCreateInfoNVBuilder {}
impl PipelineMultisampleStateCreateInfoNext for vk::PipelineCoverageReductionStateCreateInfoNV {}
impl Builder<'_> for vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    type Type = PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder {
    inner: vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL,
}
impl PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_integer_functions2(mut self, shader_integer_functions2: bool) -> Self {
        self.inner.shader_integer_functions2 = if shader_integer_functions2 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder {
    type Target = vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {}
impl Builder<'_> for vk::InitializePerformanceApiInfoINTEL {
    type Type = InitializePerformanceApiInfoINTELBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct InitializePerformanceApiInfoINTELBuilder {
    inner: vk::InitializePerformanceApiInfoINTEL,
}
impl InitializePerformanceApiInfoINTELBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_user_data(mut self, p_user_data: *mut c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
}
impl Deref for InitializePerformanceApiInfoINTELBuilder {
    type Target = vk::InitializePerformanceApiInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::QueryPoolPerformanceQueryCreateInfoINTEL {
    type Type = QueryPoolPerformanceQueryCreateInfoINTELBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct QueryPoolPerformanceQueryCreateInfoINTELBuilder {
    inner: vk::QueryPoolPerformanceQueryCreateInfoINTEL,
}
impl QueryPoolPerformanceQueryCreateInfoINTELBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn performance_counters_sampling(
        mut self,
        performance_counters_sampling: vk::QueryPoolSamplingModeINTEL,
    ) -> Self {
        self.inner.performance_counters_sampling = performance_counters_sampling;
        self
    }
}
impl Deref for QueryPoolPerformanceQueryCreateInfoINTELBuilder {
    type Target = vk::QueryPoolPerformanceQueryCreateInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl QueryPoolCreateInfoNext for QueryPoolPerformanceQueryCreateInfoINTELBuilder {}
impl QueryPoolCreateInfoNext for vk::QueryPoolPerformanceQueryCreateInfoINTEL {}
impl Builder<'_> for vk::PerformanceMarkerInfoINTEL {
    type Type = PerformanceMarkerInfoINTELBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PerformanceMarkerInfoINTELBuilder {
    inner: vk::PerformanceMarkerInfoINTEL,
}
impl PerformanceMarkerInfoINTELBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn marker(mut self, marker: u64) -> Self {
        self.inner.marker = marker;
        self
    }
}
impl Deref for PerformanceMarkerInfoINTELBuilder {
    type Target = vk::PerformanceMarkerInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PerformanceStreamMarkerInfoINTEL {
    type Type = PerformanceStreamMarkerInfoINTELBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PerformanceStreamMarkerInfoINTELBuilder {
    inner: vk::PerformanceStreamMarkerInfoINTEL,
}
impl PerformanceStreamMarkerInfoINTELBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn marker(mut self, marker: u32) -> Self {
        self.inner.marker = marker;
        self
    }
}
impl Deref for PerformanceStreamMarkerInfoINTELBuilder {
    type Target = vk::PerformanceStreamMarkerInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PerformanceOverrideInfoINTEL {
    type Type = PerformanceOverrideInfoINTELBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PerformanceOverrideInfoINTELBuilder {
    inner: vk::PerformanceOverrideInfoINTEL,
}
impl PerformanceOverrideInfoINTELBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::PerformanceOverrideTypeINTEL) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn enable(mut self, enable: bool) -> Self {
        self.inner.enable = if enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn parameter(mut self, parameter: u64) -> Self {
        self.inner.parameter = parameter;
        self
    }
}
impl Deref for PerformanceOverrideInfoINTELBuilder {
    type Target = vk::PerformanceOverrideInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PerformanceConfigurationAcquireInfoINTEL {
    type Type = PerformanceConfigurationAcquireInfoINTELBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PerformanceConfigurationAcquireInfoINTELBuilder {
    inner: vk::PerformanceConfigurationAcquireInfoINTEL,
}
impl PerformanceConfigurationAcquireInfoINTELBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::PerformanceConfigurationTypeINTEL) -> Self {
        self.inner.ty = ty;
        self
    }
}
impl Deref for PerformanceConfigurationAcquireInfoINTELBuilder {
    type Target = vk::PerformanceConfigurationAcquireInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderClockFeaturesKHR {
    type Type = PhysicalDeviceShaderClockFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderClockFeaturesKHRBuilder {
    inner: vk::PhysicalDeviceShaderClockFeaturesKHR,
}
impl PhysicalDeviceShaderClockFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_subgroup_clock(mut self, shader_subgroup_clock: bool) -> Self {
        self.inner.shader_subgroup_clock = if shader_subgroup_clock { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_device_clock(mut self, shader_device_clock: bool) -> Self {
        self.inner.shader_device_clock = if shader_device_clock { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceShaderClockFeaturesKHRBuilder {
    type Target = vk::PhysicalDeviceShaderClockFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceShaderClockFeaturesKHRBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceShaderClockFeaturesKHRBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderClockFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderClockFeaturesKHR {}
impl Builder<'_> for vk::PhysicalDeviceIndexTypeUint8FeaturesEXT {
    type Type = PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder {
    inner: vk::PhysicalDeviceIndexTypeUint8FeaturesEXT,
}
impl PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn index_type_uint8(mut self, index_type_uint8: bool) -> Self {
        self.inner.index_type_uint8 = if index_type_uint8 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceIndexTypeUint8FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceIndexTypeUint8FeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceIndexTypeUint8FeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceShaderSMBuiltinsPropertiesNV {}
impl Builder<'_> for vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    type Type = PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder {
    inner: vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV,
}
impl PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_sm_builtins(mut self, shader_sm_builtins: bool) -> Self {
        self.inner.shader_sm_builtins = if shader_sm_builtins { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV {}
impl Builder<'_> for vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    type Type = PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT,
}
impl PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_shader_sample_interlock(mut self, fragment_shader_sample_interlock: bool) -> Self {
        self.inner.fragment_shader_sample_interlock = if fragment_shader_sample_interlock {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn fragment_shader_pixel_interlock(mut self, fragment_shader_pixel_interlock: bool) -> Self {
        self.inner.fragment_shader_pixel_interlock = if fragment_shader_pixel_interlock {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn fragment_shader_shading_rate_interlock(mut self, fragment_shader_shading_rate_interlock: bool) -> Self {
        self.inner.fragment_shader_shading_rate_interlock = if fragment_shader_shading_rate_interlock {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT {}
impl Builder<'_> for vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    type Type = PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder {
    inner: vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures,
}
impl PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn separate_depth_stencil_layouts(mut self, separate_depth_stencil_layouts: bool) -> Self {
        self.inner.separate_depth_stencil_layouts = if separate_depth_stencil_layouts {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder {
    type Target = vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures {}
impl Builder<'_> for vk::AttachmentReferenceStencilLayout {
    type Type = AttachmentReferenceStencilLayoutBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AttachmentReferenceStencilLayoutBuilder {
    inner: vk::AttachmentReferenceStencilLayout,
}
impl AttachmentReferenceStencilLayoutBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn stencil_layout(mut self, stencil_layout: vk::ImageLayout) -> Self {
        self.inner.stencil_layout = stencil_layout;
        self
    }
}
impl Deref for AttachmentReferenceStencilLayoutBuilder {
    type Target = vk::AttachmentReferenceStencilLayout;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl AttachmentReference2Next for AttachmentReferenceStencilLayoutBuilder {}
impl AttachmentReference2Next for vk::AttachmentReferenceStencilLayout {}
impl Builder<'_> for vk::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    type Type = PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder {
    inner: vk::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT,
}
impl PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn primitive_topology_list_restart(mut self, primitive_topology_list_restart: bool) -> Self {
        self.inner.primitive_topology_list_restart = if primitive_topology_list_restart {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn primitive_topology_patch_list_restart(mut self, primitive_topology_patch_list_restart: bool) -> Self {
        self.inner.primitive_topology_patch_list_restart = if primitive_topology_patch_list_restart {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder {
    type Target = vk::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {}
impl Builder<'_> for vk::AttachmentDescriptionStencilLayout {
    type Type = AttachmentDescriptionStencilLayoutBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AttachmentDescriptionStencilLayoutBuilder {
    inner: vk::AttachmentDescriptionStencilLayout,
}
impl AttachmentDescriptionStencilLayoutBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn stencil_initial_layout(mut self, stencil_initial_layout: vk::ImageLayout) -> Self {
        self.inner.stencil_initial_layout = stencil_initial_layout;
        self
    }
    pub fn stencil_final_layout(mut self, stencil_final_layout: vk::ImageLayout) -> Self {
        self.inner.stencil_final_layout = stencil_final_layout;
        self
    }
}
impl Deref for AttachmentDescriptionStencilLayoutBuilder {
    type Target = vk::AttachmentDescriptionStencilLayout;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl AttachmentDescription2Next for AttachmentDescriptionStencilLayoutBuilder {}
impl AttachmentDescription2Next for vk::AttachmentDescriptionStencilLayout {}
impl Builder<'_> for vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    type Type = PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder {
    inner: vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR,
}
impl PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline_executable_info(mut self, pipeline_executable_info: bool) -> Self {
        self.inner.pipeline_executable_info = if pipeline_executable_info { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder {
    type Target = vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder {}
impl DeviceCreateInfoNext for PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {}
impl Builder<'_> for vk::PipelineInfoKHR {
    type Type = PipelineInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineInfoKHRBuilder {
    inner: vk::PipelineInfoKHR,
}
impl PipelineInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline(mut self, pipeline: vk::Pipeline) -> Self {
        self.inner.pipeline = Some(pipeline);
        self
    }
}
impl Deref for PipelineInfoKHRBuilder {
    type Target = vk::PipelineInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineExecutableInfoKHR {
    type Type = PipelineExecutableInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineExecutableInfoKHRBuilder {
    inner: vk::PipelineExecutableInfoKHR,
}
impl PipelineExecutableInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline(mut self, pipeline: vk::Pipeline) -> Self {
        self.inner.pipeline = Some(pipeline);
        self
    }
    pub fn executable_index(mut self, executable_index: u32) -> Self {
        self.inner.executable_index = executable_index;
        self
    }
}
impl Deref for PipelineExecutableInfoKHRBuilder {
    type Target = vk::PipelineExecutableInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    type Type = PhysicalDeviceShaderDemoteToHelperInvocationFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeaturesBuilder {
    inner: vk::PhysicalDeviceShaderDemoteToHelperInvocationFeatures,
}
impl PhysicalDeviceShaderDemoteToHelperInvocationFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_demote_to_helper_invocation(mut self, shader_demote_to_helper_invocation: bool) -> Self {
        self.inner.shader_demote_to_helper_invocation = if shader_demote_to_helper_invocation {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShaderDemoteToHelperInvocationFeaturesBuilder {
    type Target = vk::PhysicalDeviceShaderDemoteToHelperInvocationFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceShaderDemoteToHelperInvocationFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceShaderDemoteToHelperInvocationFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderDemoteToHelperInvocationFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderDemoteToHelperInvocationFeatures {}
impl Builder<'_> for vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    type Type = PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT,
}
impl PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn texel_buffer_alignment(mut self, texel_buffer_alignment: bool) -> Self {
        self.inner.texel_buffer_alignment = if texel_buffer_alignment { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceTexelBufferAlignmentProperties {}
impl Builder<'_> for vk::PhysicalDeviceSubgroupSizeControlFeatures {
    type Type = PhysicalDeviceSubgroupSizeControlFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceSubgroupSizeControlFeaturesBuilder {
    inner: vk::PhysicalDeviceSubgroupSizeControlFeatures,
}
impl PhysicalDeviceSubgroupSizeControlFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn subgroup_size_control(mut self, subgroup_size_control: bool) -> Self {
        self.inner.subgroup_size_control = if subgroup_size_control { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn compute_full_subgroups(mut self, compute_full_subgroups: bool) -> Self {
        self.inner.compute_full_subgroups = if compute_full_subgroups { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceSubgroupSizeControlFeaturesBuilder {
    type Target = vk::PhysicalDeviceSubgroupSizeControlFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceSubgroupSizeControlFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceSubgroupSizeControlFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceSubgroupSizeControlFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceSubgroupSizeControlFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceSubgroupSizeControlProperties {}
impl PipelineShaderStageCreateInfoNext for vk::PipelineShaderStageRequiredSubgroupSizeCreateInfo {}
impl ComputePipelineCreateInfoNext for vk::SubpassShadingPipelineCreateInfoHUAWEI {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceSubpassShadingPropertiesHUAWEI {}
impl Builder<'_> for vk::MemoryOpaqueCaptureAddressAllocateInfo {
    type Type = MemoryOpaqueCaptureAddressAllocateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryOpaqueCaptureAddressAllocateInfoBuilder {
    inner: vk::MemoryOpaqueCaptureAddressAllocateInfo,
}
impl MemoryOpaqueCaptureAddressAllocateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn opaque_capture_address(mut self, opaque_capture_address: u64) -> Self {
        self.inner.opaque_capture_address = opaque_capture_address;
        self
    }
}
impl Deref for MemoryOpaqueCaptureAddressAllocateInfoBuilder {
    type Target = vk::MemoryOpaqueCaptureAddressAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for MemoryOpaqueCaptureAddressAllocateInfoBuilder {}
impl MemoryAllocateInfoNext for vk::MemoryOpaqueCaptureAddressAllocateInfo {}
impl Builder<'_> for vk::DeviceMemoryOpaqueCaptureAddressInfo {
    type Type = DeviceMemoryOpaqueCaptureAddressInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceMemoryOpaqueCaptureAddressInfoBuilder {
    inner: vk::DeviceMemoryOpaqueCaptureAddressInfo,
}
impl DeviceMemoryOpaqueCaptureAddressInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
}
impl Deref for DeviceMemoryOpaqueCaptureAddressInfoBuilder {
    type Target = vk::DeviceMemoryOpaqueCaptureAddressInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceLineRasterizationFeaturesEXT {
    type Type = PhysicalDeviceLineRasterizationFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceLineRasterizationFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceLineRasterizationFeaturesEXT,
}
impl PhysicalDeviceLineRasterizationFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn rectangular_lines(mut self, rectangular_lines: bool) -> Self {
        self.inner.rectangular_lines = if rectangular_lines { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn bresenham_lines(mut self, bresenham_lines: bool) -> Self {
        self.inner.bresenham_lines = if bresenham_lines { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn smooth_lines(mut self, smooth_lines: bool) -> Self {
        self.inner.smooth_lines = if smooth_lines { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn stippled_rectangular_lines(mut self, stippled_rectangular_lines: bool) -> Self {
        self.inner.stippled_rectangular_lines = if stippled_rectangular_lines {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn stippled_bresenham_lines(mut self, stippled_bresenham_lines: bool) -> Self {
        self.inner.stippled_bresenham_lines = if stippled_bresenham_lines { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn stippled_smooth_lines(mut self, stippled_smooth_lines: bool) -> Self {
        self.inner.stippled_smooth_lines = if stippled_smooth_lines { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceLineRasterizationFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceLineRasterizationFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceLineRasterizationFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceLineRasterizationFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceLineRasterizationFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceLineRasterizationFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceLineRasterizationPropertiesEXT {}
impl Builder<'_> for vk::PipelineRasterizationLineStateCreateInfoEXT {
    type Type = PipelineRasterizationLineStateCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationLineStateCreateInfoEXTBuilder {
    inner: vk::PipelineRasterizationLineStateCreateInfoEXT,
}
impl PipelineRasterizationLineStateCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn line_rasterization_mode(mut self, line_rasterization_mode: vk::LineRasterizationModeEXT) -> Self {
        self.inner.line_rasterization_mode = line_rasterization_mode;
        self
    }
    pub fn stippled_line_enable(mut self, stippled_line_enable: bool) -> Self {
        self.inner.stippled_line_enable = if stippled_line_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn line_stipple_factor(mut self, line_stipple_factor: u32) -> Self {
        self.inner.line_stipple_factor = line_stipple_factor;
        self
    }
    pub fn line_stipple_pattern(mut self, line_stipple_pattern: u16) -> Self {
        self.inner.line_stipple_pattern = line_stipple_pattern;
        self
    }
}
impl Deref for PipelineRasterizationLineStateCreateInfoEXTBuilder {
    type Target = vk::PipelineRasterizationLineStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineRasterizationStateCreateInfoNext for PipelineRasterizationLineStateCreateInfoEXTBuilder {}
impl PipelineRasterizationStateCreateInfoNext for vk::PipelineRasterizationLineStateCreateInfoEXT {}
impl Builder<'_> for vk::PhysicalDevicePipelineCreationCacheControlFeatures {
    type Type = PhysicalDevicePipelineCreationCacheControlFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePipelineCreationCacheControlFeaturesBuilder {
    inner: vk::PhysicalDevicePipelineCreationCacheControlFeatures,
}
impl PhysicalDevicePipelineCreationCacheControlFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline_creation_cache_control(mut self, pipeline_creation_cache_control: bool) -> Self {
        self.inner.pipeline_creation_cache_control = if pipeline_creation_cache_control {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDevicePipelineCreationCacheControlFeaturesBuilder {
    type Target = vk::PhysicalDevicePipelineCreationCacheControlFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDevicePipelineCreationCacheControlFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDevicePipelineCreationCacheControlFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePipelineCreationCacheControlFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePipelineCreationCacheControlFeatures {}
impl Builder<'_> for vk::PhysicalDeviceVulkan11Features {
    type Type = PhysicalDeviceVulkan11FeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVulkan11FeaturesBuilder {
    inner: vk::PhysicalDeviceVulkan11Features,
}
impl PhysicalDeviceVulkan11FeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn storage_buffer16_bit_access(mut self, storage_buffer16_bit_access: bool) -> Self {
        self.inner.storage_buffer16_bit_access = if storage_buffer16_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn uniform_and_storage_buffer16_bit_access(mut self, uniform_and_storage_buffer16_bit_access: bool) -> Self {
        self.inner.uniform_and_storage_buffer16_bit_access = if uniform_and_storage_buffer16_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn storage_push_constant16(mut self, storage_push_constant16: bool) -> Self {
        self.inner.storage_push_constant16 = if storage_push_constant16 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn storage_input_output16(mut self, storage_input_output16: bool) -> Self {
        self.inner.storage_input_output16 = if storage_input_output16 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multiview(mut self, multiview: bool) -> Self {
        self.inner.multiview = if multiview { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multiview_geometry_shader(mut self, multiview_geometry_shader: bool) -> Self {
        self.inner.multiview_geometry_shader = if multiview_geometry_shader { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multiview_tessellation_shader(mut self, multiview_tessellation_shader: bool) -> Self {
        self.inner.multiview_tessellation_shader = if multiview_tessellation_shader {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn variable_pointers_storage_buffer(mut self, variable_pointers_storage_buffer: bool) -> Self {
        self.inner.variable_pointers_storage_buffer = if variable_pointers_storage_buffer {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn variable_pointers(mut self, variable_pointers: bool) -> Self {
        self.inner.variable_pointers = if variable_pointers { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn protected_memory(mut self, protected_memory: bool) -> Self {
        self.inner.protected_memory = if protected_memory { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn sampler_ycbcr_conversion(mut self, sampler_ycbcr_conversion: bool) -> Self {
        self.inner.sampler_ycbcr_conversion = if sampler_ycbcr_conversion { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_draw_parameters(mut self, shader_draw_parameters: bool) -> Self {
        self.inner.shader_draw_parameters = if shader_draw_parameters { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceVulkan11FeaturesBuilder {
    type Target = vk::PhysicalDeviceVulkan11Features;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceVulkan11FeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceVulkan11FeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceVulkan11Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceVulkan11Features {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceVulkan11Properties {}
impl Builder<'_> for vk::PhysicalDeviceVulkan12Features {
    type Type = PhysicalDeviceVulkan12FeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVulkan12FeaturesBuilder {
    inner: vk::PhysicalDeviceVulkan12Features,
}
impl PhysicalDeviceVulkan12FeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn sampler_mirror_clamp_to_edge(mut self, sampler_mirror_clamp_to_edge: bool) -> Self {
        self.inner.sampler_mirror_clamp_to_edge = if sampler_mirror_clamp_to_edge {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn draw_indirect_count(mut self, draw_indirect_count: bool) -> Self {
        self.inner.draw_indirect_count = if draw_indirect_count { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn storage_buffer8_bit_access(mut self, storage_buffer8_bit_access: bool) -> Self {
        self.inner.storage_buffer8_bit_access = if storage_buffer8_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn uniform_and_storage_buffer8_bit_access(mut self, uniform_and_storage_buffer8_bit_access: bool) -> Self {
        self.inner.uniform_and_storage_buffer8_bit_access = if uniform_and_storage_buffer8_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn storage_push_constant8(mut self, storage_push_constant8: bool) -> Self {
        self.inner.storage_push_constant8 = if storage_push_constant8 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_buffer_int64_atomics(mut self, shader_buffer_int64_atomics: bool) -> Self {
        self.inner.shader_buffer_int64_atomics = if shader_buffer_int64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_int64_atomics(mut self, shader_shared_int64_atomics: bool) -> Self {
        self.inner.shader_shared_int64_atomics = if shader_shared_int64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_float16(mut self, shader_float16: bool) -> Self {
        self.inner.shader_float16 = if shader_float16 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_int8(mut self, shader_int8: bool) -> Self {
        self.inner.shader_int8 = if shader_int8 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn descriptor_indexing(mut self, descriptor_indexing: bool) -> Self {
        self.inner.descriptor_indexing = if descriptor_indexing { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_input_attachment_array_dynamic_indexing(
        mut self,
        shader_input_attachment_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_input_attachment_array_dynamic_indexing = if shader_input_attachment_array_dynamic_indexing {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_texel_buffer_array_dynamic_indexing =
            if shader_uniform_texel_buffer_array_dynamic_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_storage_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_texel_buffer_array_dynamic_indexing =
            if shader_storage_texel_buffer_array_dynamic_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_uniform_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_buffer_array_non_uniform_indexing =
            if shader_uniform_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_sampled_image_array_non_uniform_indexing(
        mut self,
        shader_sampled_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_sampled_image_array_non_uniform_indexing = if shader_sampled_image_array_non_uniform_indexing
        {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_storage_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_buffer_array_non_uniform_indexing =
            if shader_storage_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_image_array_non_uniform_indexing(
        mut self,
        shader_storage_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_image_array_non_uniform_indexing = if shader_storage_image_array_non_uniform_indexing
        {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_input_attachment_array_non_uniform_indexing(
        mut self,
        shader_input_attachment_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_input_attachment_array_non_uniform_indexing =
            if shader_input_attachment_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_texel_buffer_array_non_uniform_indexing =
            if shader_uniform_texel_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_texel_buffer_array_non_uniform_indexing =
            if shader_storage_texel_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_uniform_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_uniform_buffer_update_after_bind =
            if descriptor_binding_uniform_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_sampled_image_update_after_bind(
        mut self,
        descriptor_binding_sampled_image_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_sampled_image_update_after_bind =
            if descriptor_binding_sampled_image_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_image_update_after_bind(
        mut self,
        descriptor_binding_storage_image_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_image_update_after_bind =
            if descriptor_binding_storage_image_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_buffer_update_after_bind =
            if descriptor_binding_storage_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_uniform_texel_buffer_update_after_bind =
            if descriptor_binding_uniform_texel_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_texel_buffer_update_after_bind =
            if descriptor_binding_storage_texel_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_update_unused_while_pending(
        mut self,
        descriptor_binding_update_unused_while_pending: bool,
    ) -> Self {
        self.inner.descriptor_binding_update_unused_while_pending = if descriptor_binding_update_unused_while_pending {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn descriptor_binding_partially_bound(mut self, descriptor_binding_partially_bound: bool) -> Self {
        self.inner.descriptor_binding_partially_bound = if descriptor_binding_partially_bound {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn descriptor_binding_variable_descriptor_count(
        mut self,
        descriptor_binding_variable_descriptor_count: bool,
    ) -> Self {
        self.inner.descriptor_binding_variable_descriptor_count = if descriptor_binding_variable_descriptor_count {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn runtime_descriptor_array(mut self, runtime_descriptor_array: bool) -> Self {
        self.inner.runtime_descriptor_array = if runtime_descriptor_array { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn sampler_filter_minmax(mut self, sampler_filter_minmax: bool) -> Self {
        self.inner.sampler_filter_minmax = if sampler_filter_minmax { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn scalar_block_layout(mut self, scalar_block_layout: bool) -> Self {
        self.inner.scalar_block_layout = if scalar_block_layout { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn imageless_framebuffer(mut self, imageless_framebuffer: bool) -> Self {
        self.inner.imageless_framebuffer = if imageless_framebuffer { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn uniform_buffer_standard_layout(mut self, uniform_buffer_standard_layout: bool) -> Self {
        self.inner.uniform_buffer_standard_layout = if uniform_buffer_standard_layout {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_subgroup_extended_types(mut self, shader_subgroup_extended_types: bool) -> Self {
        self.inner.shader_subgroup_extended_types = if shader_subgroup_extended_types {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn separate_depth_stencil_layouts(mut self, separate_depth_stencil_layouts: bool) -> Self {
        self.inner.separate_depth_stencil_layouts = if separate_depth_stencil_layouts {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn host_query_reset(mut self, host_query_reset: bool) -> Self {
        self.inner.host_query_reset = if host_query_reset { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn timeline_semaphore(mut self, timeline_semaphore: bool) -> Self {
        self.inner.timeline_semaphore = if timeline_semaphore { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn buffer_device_address(mut self, buffer_device_address: bool) -> Self {
        self.inner.buffer_device_address = if buffer_device_address { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn buffer_device_address_capture_replay(mut self, buffer_device_address_capture_replay: bool) -> Self {
        self.inner.buffer_device_address_capture_replay = if buffer_device_address_capture_replay {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn buffer_device_address_multi_device(mut self, buffer_device_address_multi_device: bool) -> Self {
        self.inner.buffer_device_address_multi_device = if buffer_device_address_multi_device {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn vulkan_memory_model(mut self, vulkan_memory_model: bool) -> Self {
        self.inner.vulkan_memory_model = if vulkan_memory_model { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn vulkan_memory_model_device_scope(mut self, vulkan_memory_model_device_scope: bool) -> Self {
        self.inner.vulkan_memory_model_device_scope = if vulkan_memory_model_device_scope {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn vulkan_memory_model_availability_visibility_chains(
        mut self,
        vulkan_memory_model_availability_visibility_chains: bool,
    ) -> Self {
        self.inner.vulkan_memory_model_availability_visibility_chains =
            if vulkan_memory_model_availability_visibility_chains {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_output_viewport_index(mut self, shader_output_viewport_index: bool) -> Self {
        self.inner.shader_output_viewport_index = if shader_output_viewport_index {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_output_layer(mut self, shader_output_layer: bool) -> Self {
        self.inner.shader_output_layer = if shader_output_layer { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn subgroup_broadcast_dynamic_id(mut self, subgroup_broadcast_dynamic_id: bool) -> Self {
        self.inner.subgroup_broadcast_dynamic_id = if subgroup_broadcast_dynamic_id {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceVulkan12FeaturesBuilder {
    type Target = vk::PhysicalDeviceVulkan12Features;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceVulkan12FeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceVulkan12FeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceVulkan12Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceVulkan12Features {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceVulkan12Properties {}
impl Builder<'_> for vk::PhysicalDeviceVulkan13Features {
    type Type = PhysicalDeviceVulkan13FeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVulkan13FeaturesBuilder {
    inner: vk::PhysicalDeviceVulkan13Features,
}
impl PhysicalDeviceVulkan13FeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn robust_image_access(mut self, robust_image_access: bool) -> Self {
        self.inner.robust_image_access = if robust_image_access { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn inline_uniform_block(mut self, inline_uniform_block: bool) -> Self {
        self.inner.inline_uniform_block = if inline_uniform_block { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn descriptor_binding_inline_uniform_block_update_after_bind(
        mut self,
        descriptor_binding_inline_uniform_block_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_inline_uniform_block_update_after_bind =
            if descriptor_binding_inline_uniform_block_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn pipeline_creation_cache_control(mut self, pipeline_creation_cache_control: bool) -> Self {
        self.inner.pipeline_creation_cache_control = if pipeline_creation_cache_control {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn private_data(mut self, private_data: bool) -> Self {
        self.inner.private_data = if private_data { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_demote_to_helper_invocation(mut self, shader_demote_to_helper_invocation: bool) -> Self {
        self.inner.shader_demote_to_helper_invocation = if shader_demote_to_helper_invocation {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_terminate_invocation(mut self, shader_terminate_invocation: bool) -> Self {
        self.inner.shader_terminate_invocation = if shader_terminate_invocation {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn subgroup_size_control(mut self, subgroup_size_control: bool) -> Self {
        self.inner.subgroup_size_control = if subgroup_size_control { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn compute_full_subgroups(mut self, compute_full_subgroups: bool) -> Self {
        self.inner.compute_full_subgroups = if compute_full_subgroups { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn synchronization2(mut self, synchronization2: bool) -> Self {
        self.inner.synchronization2 = if synchronization2 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn texture_compression_astc_hdr(mut self, texture_compression_astc_hdr: bool) -> Self {
        self.inner.texture_compression_astc_hdr = if texture_compression_astc_hdr {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_zero_initialize_workgroup_memory(mut self, shader_zero_initialize_workgroup_memory: bool) -> Self {
        self.inner.shader_zero_initialize_workgroup_memory = if shader_zero_initialize_workgroup_memory {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn dynamic_rendering(mut self, dynamic_rendering: bool) -> Self {
        self.inner.dynamic_rendering = if dynamic_rendering { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_integer_dot_product(mut self, shader_integer_dot_product: bool) -> Self {
        self.inner.shader_integer_dot_product = if shader_integer_dot_product {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn maintenance4(mut self, maintenance4: bool) -> Self {
        self.inner.maintenance4 = if maintenance4 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceVulkan13FeaturesBuilder {
    type Target = vk::PhysicalDeviceVulkan13Features;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceVulkan13FeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceVulkan13FeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceVulkan13Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceVulkan13Features {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceVulkan13Properties {}
impl Builder<'_> for vk::PipelineCompilerControlCreateInfoAMD {
    type Type = PipelineCompilerControlCreateInfoAMDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCompilerControlCreateInfoAMDBuilder {
    inner: vk::PipelineCompilerControlCreateInfoAMD,
}
impl PipelineCompilerControlCreateInfoAMDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn compiler_control_flags(mut self, compiler_control_flags: vk::PipelineCompilerControlFlagsAMD) -> Self {
        self.inner.compiler_control_flags = compiler_control_flags;
        self
    }
}
impl Deref for PipelineCompilerControlCreateInfoAMDBuilder {
    type Target = vk::PipelineCompilerControlCreateInfoAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl GraphicsPipelineCreateInfoNext for PipelineCompilerControlCreateInfoAMDBuilder {}
impl ComputePipelineCreateInfoNext for PipelineCompilerControlCreateInfoAMDBuilder {}
impl GraphicsPipelineCreateInfoNext for vk::PipelineCompilerControlCreateInfoAMD {}
impl ComputePipelineCreateInfoNext for vk::PipelineCompilerControlCreateInfoAMD {}
impl Builder<'_> for vk::PhysicalDeviceCoherentMemoryFeaturesAMD {
    type Type = PhysicalDeviceCoherentMemoryFeaturesAMDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceCoherentMemoryFeaturesAMDBuilder {
    inner: vk::PhysicalDeviceCoherentMemoryFeaturesAMD,
}
impl PhysicalDeviceCoherentMemoryFeaturesAMDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_coherent_memory(mut self, device_coherent_memory: bool) -> Self {
        self.inner.device_coherent_memory = if device_coherent_memory { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceCoherentMemoryFeaturesAMDBuilder {
    type Target = vk::PhysicalDeviceCoherentMemoryFeaturesAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceCoherentMemoryFeaturesAMDBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceCoherentMemoryFeaturesAMDBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCoherentMemoryFeaturesAMD {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCoherentMemoryFeaturesAMD {}
impl Builder<'_> for vk::SamplerCustomBorderColorCreateInfoEXT {
    type Type = SamplerCustomBorderColorCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SamplerCustomBorderColorCreateInfoEXTBuilder {
    inner: vk::SamplerCustomBorderColorCreateInfoEXT,
}
impl SamplerCustomBorderColorCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn custom_border_color(mut self, custom_border_color: vk::ClearColorValue) -> Self {
        self.inner.custom_border_color = custom_border_color;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
}
impl Deref for SamplerCustomBorderColorCreateInfoEXTBuilder {
    type Target = vk::SamplerCustomBorderColorCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SamplerCreateInfoNext for SamplerCustomBorderColorCreateInfoEXTBuilder {}
impl SamplerCreateInfoNext for vk::SamplerCustomBorderColorCreateInfoEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceCustomBorderColorPropertiesEXT {}
impl Builder<'_> for vk::PhysicalDeviceCustomBorderColorFeaturesEXT {
    type Type = PhysicalDeviceCustomBorderColorFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceCustomBorderColorFeaturesEXT,
}
impl PhysicalDeviceCustomBorderColorFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn custom_border_colors(mut self, custom_border_colors: bool) -> Self {
        self.inner.custom_border_colors = if custom_border_colors { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn custom_border_color_without_format(mut self, custom_border_color_without_format: bool) -> Self {
        self.inner.custom_border_color_without_format = if custom_border_color_without_format {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceCustomBorderColorFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceCustomBorderColorFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceCustomBorderColorFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceCustomBorderColorFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCustomBorderColorFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCustomBorderColorFeaturesEXT {}
impl Builder<'_> for vk::SamplerBorderColorComponentMappingCreateInfoEXT {
    type Type = SamplerBorderColorComponentMappingCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SamplerBorderColorComponentMappingCreateInfoEXTBuilder {
    inner: vk::SamplerBorderColorComponentMappingCreateInfoEXT,
}
impl SamplerBorderColorComponentMappingCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn components(mut self, components: vk::ComponentMapping) -> Self {
        self.inner.components = components;
        self
    }
    pub fn srgb(mut self, srgb: bool) -> Self {
        self.inner.srgb = if srgb { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for SamplerBorderColorComponentMappingCreateInfoEXTBuilder {
    type Target = vk::SamplerBorderColorComponentMappingCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SamplerCreateInfoNext for SamplerBorderColorComponentMappingCreateInfoEXTBuilder {}
impl SamplerCreateInfoNext for vk::SamplerBorderColorComponentMappingCreateInfoEXT {}
impl Builder<'_> for vk::PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    type Type = PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceBorderColorSwizzleFeaturesEXT,
}
impl PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn border_color_swizzle(mut self, border_color_swizzle: bool) -> Self {
        self.inner.border_color_swizzle = if border_color_swizzle { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn border_color_swizzle_from_image(mut self, border_color_swizzle_from_image: bool) -> Self {
        self.inner.border_color_swizzle_from_image = if border_color_swizzle_from_image {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceBorderColorSwizzleFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceBorderColorSwizzleFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceBorderColorSwizzleFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceBorderColorSwizzleFeaturesEXT {}
impl<'a> Builder<'a> for vk::AccelerationStructureGeometryTrianglesDataKHR {
    type Type = AccelerationStructureGeometryTrianglesDataKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait AccelerationStructureGeometryTrianglesDataKHRNext {}
#[derive(Default)]
pub struct AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    inner: vk::AccelerationStructureGeometryTrianglesDataKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    pub fn insert_next<T: AccelerationStructureGeometryTrianglesDataKHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn vertex_format(mut self, vertex_format: vk::Format) -> Self {
        self.inner.vertex_format = vertex_format;
        self
    }
    pub fn vertex_data(mut self, vertex_data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.vertex_data = vertex_data;
        self
    }
    pub fn vertex_stride(mut self, vertex_stride: vk::DeviceSize) -> Self {
        self.inner.vertex_stride = vertex_stride;
        self
    }
    pub fn max_vertex(mut self, max_vertex: u32) -> Self {
        self.inner.max_vertex = max_vertex;
        self
    }
    pub fn index_type(mut self, index_type: vk::IndexType) -> Self {
        self.inner.index_type = index_type;
        self
    }
    pub fn index_data(mut self, index_data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.index_data = index_data;
        self
    }
    pub fn transform_data(mut self, transform_data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.transform_data = transform_data;
        self
    }
}
impl<'a> Deref for AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    type Target = vk::AccelerationStructureGeometryTrianglesDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AccelerationStructureGeometryAabbsDataKHR {
    type Type = AccelerationStructureGeometryAabbsDataKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureGeometryAabbsDataKHRBuilder {
    inner: vk::AccelerationStructureGeometryAabbsDataKHR,
}
impl AccelerationStructureGeometryAabbsDataKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn data(mut self, data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.data = data;
        self
    }
    pub fn stride(mut self, stride: vk::DeviceSize) -> Self {
        self.inner.stride = stride;
        self
    }
}
impl Deref for AccelerationStructureGeometryAabbsDataKHRBuilder {
    type Target = vk::AccelerationStructureGeometryAabbsDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AccelerationStructureGeometryInstancesDataKHR {
    type Type = AccelerationStructureGeometryInstancesDataKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureGeometryInstancesDataKHRBuilder {
    inner: vk::AccelerationStructureGeometryInstancesDataKHR,
}
impl AccelerationStructureGeometryInstancesDataKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn array_of_pointers(mut self, array_of_pointers: bool) -> Self {
        self.inner.array_of_pointers = if array_of_pointers { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn data(mut self, data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.data = data;
        self
    }
}
impl Deref for AccelerationStructureGeometryInstancesDataKHRBuilder {
    type Target = vk::AccelerationStructureGeometryInstancesDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AccelerationStructureGeometryKHR {
    type Type = AccelerationStructureGeometryKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureGeometryKHRBuilder {
    inner: vk::AccelerationStructureGeometryKHR,
}
impl AccelerationStructureGeometryKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn geometry_type(mut self, geometry_type: vk::GeometryTypeKHR) -> Self {
        self.inner.geometry_type = geometry_type;
        self
    }
    pub fn geometry(mut self, geometry: vk::AccelerationStructureGeometryDataKHR) -> Self {
        self.inner.geometry = geometry;
        self
    }
    pub fn flags(mut self, flags: vk::GeometryFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for AccelerationStructureGeometryKHRBuilder {
    type Target = vk::AccelerationStructureGeometryKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AccelerationStructureBuildGeometryInfoKHR {
    type Type = AccelerationStructureBuildGeometryInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    inner: vk::AccelerationStructureBuildGeometryInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::AccelerationStructureTypeKHR) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn flags(mut self, flags: vk::BuildAccelerationStructureFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn mode(mut self, mode: vk::BuildAccelerationStructureModeKHR) -> Self {
        self.inner.mode = mode;
        self
    }
    pub fn src_acceleration_structure(
        mut self,
        src_acceleration_structure: Option<vk::AccelerationStructureKHR>,
    ) -> Self {
        self.inner.src_acceleration_structure = src_acceleration_structure;
        self
    }
    pub fn dst_acceleration_structure(
        mut self,
        dst_acceleration_structure: Option<vk::AccelerationStructureKHR>,
    ) -> Self {
        self.inner.dst_acceleration_structure = dst_acceleration_structure;
        self
    }
    pub fn p_geometries(mut self, p_geometries: &'a [vk::AccelerationStructureGeometryKHR]) -> Self {
        self.inner.geometry_count = p_geometries.len() as u32;
        self.inner.p_geometries = p_geometries.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn pp_geometries(mut self, pp_geometries: &'a [*const vk::AccelerationStructureGeometryKHR]) -> Self {
        self.inner.geometry_count = pp_geometries.len() as u32;
        self.inner.pp_geometries = pp_geometries.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn scratch_data(mut self, scratch_data: vk::DeviceOrHostAddressKHR) -> Self {
        self.inner.scratch_data = scratch_data;
        self
    }
}
impl<'a> Deref for AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    type Target = vk::AccelerationStructureBuildGeometryInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AccelerationStructureCreateInfoKHR {
    type Type = AccelerationStructureCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait AccelerationStructureCreateInfoKHRNext {}
#[derive(Default)]
pub struct AccelerationStructureCreateInfoKHRBuilder<'a> {
    inner: vk::AccelerationStructureCreateInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> AccelerationStructureCreateInfoKHRBuilder<'a> {
    pub fn insert_next<T: AccelerationStructureCreateInfoKHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn create_flags(mut self, create_flags: vk::AccelerationStructureCreateFlagsKHR) -> Self {
        self.inner.create_flags = create_flags;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
    pub fn ty(mut self, ty: vk::AccelerationStructureTypeKHR) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn device_address(mut self, device_address: vk::DeviceAddress) -> Self {
        self.inner.device_address = device_address;
        self
    }
}
impl<'a> Deref for AccelerationStructureCreateInfoKHRBuilder<'a> {
    type Target = vk::AccelerationStructureCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AccelerationStructureDeviceAddressInfoKHR {
    type Type = AccelerationStructureDeviceAddressInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureDeviceAddressInfoKHRBuilder {
    inner: vk::AccelerationStructureDeviceAddressInfoKHR,
}
impl AccelerationStructureDeviceAddressInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn acceleration_structure(mut self, acceleration_structure: vk::AccelerationStructureKHR) -> Self {
        self.inner.acceleration_structure = Some(acceleration_structure);
        self
    }
}
impl Deref for AccelerationStructureDeviceAddressInfoKHRBuilder {
    type Target = vk::AccelerationStructureDeviceAddressInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AccelerationStructureVersionInfoKHR {
    type Type = AccelerationStructureVersionInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureVersionInfoKHRBuilder {
    inner: vk::AccelerationStructureVersionInfoKHR,
}
impl AccelerationStructureVersionInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_version_data(mut self, p_version_data: *const u8) -> Self {
        self.inner.p_version_data = p_version_data;
        self
    }
}
impl Deref for AccelerationStructureVersionInfoKHRBuilder {
    type Target = vk::AccelerationStructureVersionInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CopyAccelerationStructureInfoKHR {
    type Type = CopyAccelerationStructureInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyAccelerationStructureInfoKHRBuilder {
    inner: vk::CopyAccelerationStructureInfoKHR,
}
impl CopyAccelerationStructureInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src(mut self, src: vk::AccelerationStructureKHR) -> Self {
        self.inner.src = Some(src);
        self
    }
    pub fn dst(mut self, dst: vk::AccelerationStructureKHR) -> Self {
        self.inner.dst = Some(dst);
        self
    }
    pub fn mode(mut self, mode: vk::CopyAccelerationStructureModeKHR) -> Self {
        self.inner.mode = mode;
        self
    }
}
impl Deref for CopyAccelerationStructureInfoKHRBuilder {
    type Target = vk::CopyAccelerationStructureInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CopyAccelerationStructureToMemoryInfoKHR {
    type Type = CopyAccelerationStructureToMemoryInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyAccelerationStructureToMemoryInfoKHRBuilder {
    inner: vk::CopyAccelerationStructureToMemoryInfoKHR,
}
impl CopyAccelerationStructureToMemoryInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src(mut self, src: vk::AccelerationStructureKHR) -> Self {
        self.inner.src = Some(src);
        self
    }
    pub fn dst(mut self, dst: vk::DeviceOrHostAddressKHR) -> Self {
        self.inner.dst = dst;
        self
    }
    pub fn mode(mut self, mode: vk::CopyAccelerationStructureModeKHR) -> Self {
        self.inner.mode = mode;
        self
    }
}
impl Deref for CopyAccelerationStructureToMemoryInfoKHRBuilder {
    type Target = vk::CopyAccelerationStructureToMemoryInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CopyMemoryToAccelerationStructureInfoKHR {
    type Type = CopyMemoryToAccelerationStructureInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyMemoryToAccelerationStructureInfoKHRBuilder {
    inner: vk::CopyMemoryToAccelerationStructureInfoKHR,
}
impl CopyMemoryToAccelerationStructureInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src(mut self, src: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.src = src;
        self
    }
    pub fn dst(mut self, dst: vk::AccelerationStructureKHR) -> Self {
        self.inner.dst = Some(dst);
        self
    }
    pub fn mode(mut self, mode: vk::CopyAccelerationStructureModeKHR) -> Self {
        self.inner.mode = mode;
        self
    }
}
impl Deref for CopyMemoryToAccelerationStructureInfoKHRBuilder {
    type Target = vk::CopyMemoryToAccelerationStructureInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::RayTracingPipelineInterfaceCreateInfoKHR {
    type Type = RayTracingPipelineInterfaceCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RayTracingPipelineInterfaceCreateInfoKHRBuilder {
    inner: vk::RayTracingPipelineInterfaceCreateInfoKHR,
}
impl RayTracingPipelineInterfaceCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn max_pipeline_ray_payload_size(mut self, max_pipeline_ray_payload_size: u32) -> Self {
        self.inner.max_pipeline_ray_payload_size = max_pipeline_ray_payload_size;
        self
    }
    pub fn max_pipeline_ray_hit_attribute_size(mut self, max_pipeline_ray_hit_attribute_size: u32) -> Self {
        self.inner.max_pipeline_ray_hit_attribute_size = max_pipeline_ray_hit_attribute_size;
        self
    }
}
impl Deref for RayTracingPipelineInterfaceCreateInfoKHRBuilder {
    type Target = vk::RayTracingPipelineInterfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineLibraryCreateInfoKHR {
    type Type = PipelineLibraryCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineLibraryCreateInfoKHRBuilder<'a> {
    inner: vk::PipelineLibraryCreateInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineLibraryCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_libraries(mut self, p_libraries: &'a [vk::Pipeline]) -> Self {
        self.inner.library_count = p_libraries.len() as u32;
        self.inner.p_libraries = p_libraries.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PipelineLibraryCreateInfoKHRBuilder<'a> {
    type Target = vk::PipelineLibraryCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> GraphicsPipelineCreateInfoNext for PipelineLibraryCreateInfoKHRBuilder<'a> {}
impl GraphicsPipelineCreateInfoNext for vk::PipelineLibraryCreateInfoKHR {}
impl Builder<'_> for vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    type Type = PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT,
}
impl PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn extended_dynamic_state(mut self, extended_dynamic_state: bool) -> Self {
        self.inner.extended_dynamic_state = if extended_dynamic_state { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT {}
impl Builder<'_> for vk::PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    type Type = PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder {
    inner: vk::PhysicalDeviceExtendedDynamicState2FeaturesEXT,
}
impl PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn extended_dynamic_state2(mut self, extended_dynamic_state2: bool) -> Self {
        self.inner.extended_dynamic_state2 = if extended_dynamic_state2 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn extended_dynamic_state2_logic_op(mut self, extended_dynamic_state2_logic_op: bool) -> Self {
        self.inner.extended_dynamic_state2_logic_op = if extended_dynamic_state2_logic_op {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn extended_dynamic_state2_patch_control_points(
        mut self,
        extended_dynamic_state2_patch_control_points: bool,
    ) -> Self {
        self.inner.extended_dynamic_state2_patch_control_points = if extended_dynamic_state2_patch_control_points {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceExtendedDynamicState2FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceExtendedDynamicState2FeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceExtendedDynamicState2FeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceExtendedDynamicState2FeaturesEXT {}
impl Builder<'_> for vk::RenderPassTransformBeginInfoQCOM {
    type Type = RenderPassTransformBeginInfoQCOMBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassTransformBeginInfoQCOMBuilder {
    inner: vk::RenderPassTransformBeginInfoQCOM,
}
impl RenderPassTransformBeginInfoQCOMBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn transform(mut self, transform: vk::SurfaceTransformFlagsKHR) -> Self {
        self.inner.transform = transform;
        self
    }
}
impl Deref for RenderPassTransformBeginInfoQCOMBuilder {
    type Target = vk::RenderPassTransformBeginInfoQCOM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl RenderPassBeginInfoNext for RenderPassTransformBeginInfoQCOMBuilder {}
impl RenderPassBeginInfoNext for vk::RenderPassTransformBeginInfoQCOM {}
impl Builder<'_> for vk::CopyCommandTransformInfoQCOM {
    type Type = CopyCommandTransformInfoQCOMBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyCommandTransformInfoQCOMBuilder {
    inner: vk::CopyCommandTransformInfoQCOM,
}
impl CopyCommandTransformInfoQCOMBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn transform(mut self, transform: vk::SurfaceTransformFlagsKHR) -> Self {
        self.inner.transform = transform;
        self
    }
}
impl Deref for CopyCommandTransformInfoQCOMBuilder {
    type Target = vk::CopyCommandTransformInfoQCOM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl BufferImageCopy2Next for CopyCommandTransformInfoQCOMBuilder {}
impl ImageBlit2Next for CopyCommandTransformInfoQCOMBuilder {}
impl BufferImageCopy2Next for vk::CopyCommandTransformInfoQCOM {}
impl ImageBlit2Next for vk::CopyCommandTransformInfoQCOM {}
impl Builder<'_> for vk::CommandBufferInheritanceRenderPassTransformInfoQCOM {
    type Type = CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder {
    inner: vk::CommandBufferInheritanceRenderPassTransformInfoQCOM,
}
impl CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn transform(mut self, transform: vk::SurfaceTransformFlagsKHR) -> Self {
        self.inner.transform = transform;
        self
    }
    pub fn render_area(mut self, render_area: vk::Rect2D) -> Self {
        self.inner.render_area = render_area;
        self
    }
}
impl Deref for CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder {
    type Target = vk::CommandBufferInheritanceRenderPassTransformInfoQCOM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl CommandBufferInheritanceInfoNext for CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder {}
impl CommandBufferInheritanceInfoNext for vk::CommandBufferInheritanceRenderPassTransformInfoQCOM {}
impl Builder<'_> for vk::PhysicalDeviceDiagnosticsConfigFeaturesNV {
    type Type = PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder {
    inner: vk::PhysicalDeviceDiagnosticsConfigFeaturesNV,
}
impl PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn diagnostics_config(mut self, diagnostics_config: bool) -> Self {
        self.inner.diagnostics_config = if diagnostics_config { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceDiagnosticsConfigFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDiagnosticsConfigFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDiagnosticsConfigFeaturesNV {}
impl Builder<'_> for vk::DeviceDiagnosticsConfigCreateInfoNV {
    type Type = DeviceDiagnosticsConfigCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceDiagnosticsConfigCreateInfoNVBuilder {
    inner: vk::DeviceDiagnosticsConfigCreateInfoNV,
}
impl DeviceDiagnosticsConfigCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DeviceDiagnosticsConfigFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for DeviceDiagnosticsConfigCreateInfoNVBuilder {
    type Target = vk::DeviceDiagnosticsConfigCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DeviceCreateInfoNext for DeviceDiagnosticsConfigCreateInfoNVBuilder {}
impl DeviceCreateInfoNext for vk::DeviceDiagnosticsConfigCreateInfoNV {}
impl Builder<'_> for vk::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    type Type = PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesBuilder {
    inner: vk::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures,
}
impl PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_zero_initialize_workgroup_memory(mut self, shader_zero_initialize_workgroup_memory: bool) -> Self {
        self.inner.shader_zero_initialize_workgroup_memory = if shader_zero_initialize_workgroup_memory {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesBuilder {
    type Target = vk::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {}
impl Builder<'_> for vk::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    type Type = PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder {
    inner: vk::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR,
}
impl PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_subgroup_uniform_control_flow(mut self, shader_subgroup_uniform_control_flow: bool) -> Self {
        self.inner.shader_subgroup_uniform_control_flow = if shader_subgroup_uniform_control_flow {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder {
    type Target = vk::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHRBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {}
impl Builder<'_> for vk::PhysicalDeviceRobustness2FeaturesEXT {
    type Type = PhysicalDeviceRobustness2FeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceRobustness2FeaturesEXTBuilder {
    inner: vk::PhysicalDeviceRobustness2FeaturesEXT,
}
impl PhysicalDeviceRobustness2FeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn robust_buffer_access2(mut self, robust_buffer_access2: bool) -> Self {
        self.inner.robust_buffer_access2 = if robust_buffer_access2 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn robust_image_access2(mut self, robust_image_access2: bool) -> Self {
        self.inner.robust_image_access2 = if robust_image_access2 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn null_descriptor(mut self, null_descriptor: bool) -> Self {
        self.inner.null_descriptor = if null_descriptor { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceRobustness2FeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceRobustness2FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceRobustness2FeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceRobustness2FeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRobustness2FeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRobustness2FeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceRobustness2PropertiesEXT {}
impl Builder<'_> for vk::PhysicalDeviceImageRobustnessFeatures {
    type Type = PhysicalDeviceImageRobustnessFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceImageRobustnessFeaturesBuilder {
    inner: vk::PhysicalDeviceImageRobustnessFeatures,
}
impl PhysicalDeviceImageRobustnessFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn robust_image_access(mut self, robust_image_access: bool) -> Self {
        self.inner.robust_image_access = if robust_image_access { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceImageRobustnessFeaturesBuilder {
    type Target = vk::PhysicalDeviceImageRobustnessFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceImageRobustnessFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceImageRobustnessFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceImageRobustnessFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceImageRobustnessFeatures {}
impl Builder<'_> for vk::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    type Type = PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder {
    inner: vk::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR,
}
impl PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn workgroup_memory_explicit_layout(mut self, workgroup_memory_explicit_layout: bool) -> Self {
        self.inner.workgroup_memory_explicit_layout = if workgroup_memory_explicit_layout {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn workgroup_memory_explicit_layout_scalar_block_layout(
        mut self,
        workgroup_memory_explicit_layout_scalar_block_layout: bool,
    ) -> Self {
        self.inner.workgroup_memory_explicit_layout_scalar_block_layout =
            if workgroup_memory_explicit_layout_scalar_block_layout {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn workgroup_memory_explicit_layout8_bit_access(
        mut self,
        workgroup_memory_explicit_layout8_bit_access: bool,
    ) -> Self {
        self.inner.workgroup_memory_explicit_layout8_bit_access = if workgroup_memory_explicit_layout8_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn workgroup_memory_explicit_layout16_bit_access(
        mut self,
        workgroup_memory_explicit_layout16_bit_access: bool,
    ) -> Self {
        self.inner.workgroup_memory_explicit_layout16_bit_access = if workgroup_memory_explicit_layout16_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder {
    type Target = vk::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHRBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {}
impl Builder<'_> for vk::PhysicalDevicePortabilitySubsetFeaturesKHR {
    type Type = PhysicalDevicePortabilitySubsetFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePortabilitySubsetFeaturesKHRBuilder {
    inner: vk::PhysicalDevicePortabilitySubsetFeaturesKHR,
}
impl PhysicalDevicePortabilitySubsetFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn constant_alpha_color_blend_factors(mut self, constant_alpha_color_blend_factors: bool) -> Self {
        self.inner.constant_alpha_color_blend_factors = if constant_alpha_color_blend_factors {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn events(mut self, events: bool) -> Self {
        self.inner.events = if events { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn image_view_format_reinterpretation(mut self, image_view_format_reinterpretation: bool) -> Self {
        self.inner.image_view_format_reinterpretation = if image_view_format_reinterpretation {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn image_view_format_swizzle(mut self, image_view_format_swizzle: bool) -> Self {
        self.inner.image_view_format_swizzle = if image_view_format_swizzle { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn image_view_2d_on_3d_image(mut self, image_view_2d_on_3d_image: bool) -> Self {
        self.inner.image_view_2d_on_3d_image = if image_view_2d_on_3d_image { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multisample_array_image(mut self, multisample_array_image: bool) -> Self {
        self.inner.multisample_array_image = if multisample_array_image { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn mutable_comparison_samplers(mut self, mutable_comparison_samplers: bool) -> Self {
        self.inner.mutable_comparison_samplers = if mutable_comparison_samplers {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn point_polygons(mut self, point_polygons: bool) -> Self {
        self.inner.point_polygons = if point_polygons { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn sampler_mip_lod_bias(mut self, sampler_mip_lod_bias: bool) -> Self {
        self.inner.sampler_mip_lod_bias = if sampler_mip_lod_bias { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn separate_stencil_mask_ref(mut self, separate_stencil_mask_ref: bool) -> Self {
        self.inner.separate_stencil_mask_ref = if separate_stencil_mask_ref { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_sample_rate_interpolation_functions(
        mut self,
        shader_sample_rate_interpolation_functions: bool,
    ) -> Self {
        self.inner.shader_sample_rate_interpolation_functions = if shader_sample_rate_interpolation_functions {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn tessellation_isolines(mut self, tessellation_isolines: bool) -> Self {
        self.inner.tessellation_isolines = if tessellation_isolines { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn tessellation_point_mode(mut self, tessellation_point_mode: bool) -> Self {
        self.inner.tessellation_point_mode = if tessellation_point_mode { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn triangle_fans(mut self, triangle_fans: bool) -> Self {
        self.inner.triangle_fans = if triangle_fans { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn vertex_attribute_access_beyond_stride(mut self, vertex_attribute_access_beyond_stride: bool) -> Self {
        self.inner.vertex_attribute_access_beyond_stride = if vertex_attribute_access_beyond_stride {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDevicePortabilitySubsetFeaturesKHRBuilder {
    type Target = vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDevicePortabilitySubsetFeaturesKHRBuilder {}
impl DeviceCreateInfoNext for PhysicalDevicePortabilitySubsetFeaturesKHRBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePortabilitySubsetFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePortabilitySubsetFeaturesKHR {}
impl Builder<'_> for vk::PhysicalDevicePortabilitySubsetPropertiesKHR {
    type Type = PhysicalDevicePortabilitySubsetPropertiesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePortabilitySubsetPropertiesKHRBuilder {
    inner: vk::PhysicalDevicePortabilitySubsetPropertiesKHR,
}
impl PhysicalDevicePortabilitySubsetPropertiesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn min_vertex_input_binding_stride_alignment(mut self, min_vertex_input_binding_stride_alignment: u32) -> Self {
        self.inner.min_vertex_input_binding_stride_alignment = min_vertex_input_binding_stride_alignment;
        self
    }
}
impl Deref for PhysicalDevicePortabilitySubsetPropertiesKHRBuilder {
    type Target = vk::PhysicalDevicePortabilitySubsetPropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceProperties2Next for PhysicalDevicePortabilitySubsetPropertiesKHRBuilder {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDevicePortabilitySubsetPropertiesKHR {}
impl Builder<'_> for vk::PhysicalDevice4444FormatsFeaturesEXT {
    type Type = PhysicalDevice4444FormatsFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevice4444FormatsFeaturesEXTBuilder {
    inner: vk::PhysicalDevice4444FormatsFeaturesEXT,
}
impl PhysicalDevice4444FormatsFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn format_a4r4g4b4(mut self, format_a4r4g4b4: bool) -> Self {
        self.inner.format_a4r4g4b4 = if format_a4r4g4b4 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn format_a4b4g4r4(mut self, format_a4b4g4r4: bool) -> Self {
        self.inner.format_a4b4g4r4 = if format_a4b4g4r4 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDevice4444FormatsFeaturesEXTBuilder {
    type Target = vk::PhysicalDevice4444FormatsFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDevice4444FormatsFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDevice4444FormatsFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevice4444FormatsFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDevice4444FormatsFeaturesEXT {}
impl Builder<'_> for vk::PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    type Type = PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder {
    inner: vk::PhysicalDeviceSubpassShadingFeaturesHUAWEI,
}
impl PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn subpass_shading(mut self, subpass_shading: bool) -> Self {
        self.inner.subpass_shading = if subpass_shading { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder {
    type Target = vk::PhysicalDeviceSubpassShadingFeaturesHUAWEI;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceSubpassShadingFeaturesHUAWEI {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceSubpassShadingFeaturesHUAWEI {}
impl Builder<'_> for vk::BufferCopy2 {
    type Type = BufferCopy2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferCopy2Builder {
    inner: vk::BufferCopy2,
}
impl BufferCopy2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_offset(mut self, src_offset: vk::DeviceSize) -> Self {
        self.inner.src_offset = src_offset;
        self
    }
    pub fn dst_offset(mut self, dst_offset: vk::DeviceSize) -> Self {
        self.inner.dst_offset = dst_offset;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
}
impl Deref for BufferCopy2Builder {
    type Target = vk::BufferCopy2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImageCopy2 {
    type Type = ImageCopy2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageCopy2Builder {
    inner: vk::ImageCopy2,
}
impl ImageCopy2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_subresource(mut self, src_subresource: vk::ImageSubresourceLayers) -> Self {
        self.inner.src_subresource = src_subresource;
        self
    }
    pub fn src_offset(mut self, src_offset: vk::Offset3D) -> Self {
        self.inner.src_offset = src_offset;
        self
    }
    pub fn dst_subresource(mut self, dst_subresource: vk::ImageSubresourceLayers) -> Self {
        self.inner.dst_subresource = dst_subresource;
        self
    }
    pub fn dst_offset(mut self, dst_offset: vk::Offset3D) -> Self {
        self.inner.dst_offset = dst_offset;
        self
    }
    pub fn extent(mut self, extent: vk::Extent3D) -> Self {
        self.inner.extent = extent;
        self
    }
}
impl Deref for ImageCopy2Builder {
    type Target = vk::ImageCopy2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageBlit2 {
    type Type = ImageBlit2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ImageBlit2Next {}
#[derive(Default)]
pub struct ImageBlit2Builder<'a> {
    inner: vk::ImageBlit2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ImageBlit2Builder<'a> {
    pub fn insert_next<T: ImageBlit2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_subresource(mut self, src_subresource: vk::ImageSubresourceLayers) -> Self {
        self.inner.src_subresource = src_subresource;
        self
    }
    pub fn dst_subresource(mut self, dst_subresource: vk::ImageSubresourceLayers) -> Self {
        self.inner.dst_subresource = dst_subresource;
        self
    }
}
impl<'a> Deref for ImageBlit2Builder<'a> {
    type Target = vk::ImageBlit2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BufferImageCopy2 {
    type Type = BufferImageCopy2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait BufferImageCopy2Next {}
#[derive(Default)]
pub struct BufferImageCopy2Builder<'a> {
    inner: vk::BufferImageCopy2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BufferImageCopy2Builder<'a> {
    pub fn insert_next<T: BufferImageCopy2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer_offset(mut self, buffer_offset: vk::DeviceSize) -> Self {
        self.inner.buffer_offset = buffer_offset;
        self
    }
    pub fn buffer_row_length(mut self, buffer_row_length: u32) -> Self {
        self.inner.buffer_row_length = buffer_row_length;
        self
    }
    pub fn buffer_image_height(mut self, buffer_image_height: u32) -> Self {
        self.inner.buffer_image_height = buffer_image_height;
        self
    }
    pub fn image_subresource(mut self, image_subresource: vk::ImageSubresourceLayers) -> Self {
        self.inner.image_subresource = image_subresource;
        self
    }
    pub fn image_offset(mut self, image_offset: vk::Offset3D) -> Self {
        self.inner.image_offset = image_offset;
        self
    }
    pub fn image_extent(mut self, image_extent: vk::Extent3D) -> Self {
        self.inner.image_extent = image_extent;
        self
    }
}
impl<'a> Deref for BufferImageCopy2Builder<'a> {
    type Target = vk::BufferImageCopy2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImageResolve2 {
    type Type = ImageResolve2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageResolve2Builder {
    inner: vk::ImageResolve2,
}
impl ImageResolve2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_subresource(mut self, src_subresource: vk::ImageSubresourceLayers) -> Self {
        self.inner.src_subresource = src_subresource;
        self
    }
    pub fn src_offset(mut self, src_offset: vk::Offset3D) -> Self {
        self.inner.src_offset = src_offset;
        self
    }
    pub fn dst_subresource(mut self, dst_subresource: vk::ImageSubresourceLayers) -> Self {
        self.inner.dst_subresource = dst_subresource;
        self
    }
    pub fn dst_offset(mut self, dst_offset: vk::Offset3D) -> Self {
        self.inner.dst_offset = dst_offset;
        self
    }
    pub fn extent(mut self, extent: vk::Extent3D) -> Self {
        self.inner.extent = extent;
        self
    }
}
impl Deref for ImageResolve2Builder {
    type Target = vk::ImageResolve2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CopyBufferInfo2 {
    type Type = CopyBufferInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyBufferInfo2Builder<'a> {
    inner: vk::CopyBufferInfo2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CopyBufferInfo2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_buffer(mut self, src_buffer: vk::Buffer) -> Self {
        self.inner.src_buffer = Some(src_buffer);
        self
    }
    pub fn dst_buffer(mut self, dst_buffer: vk::Buffer) -> Self {
        self.inner.dst_buffer = Some(dst_buffer);
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::BufferCopy2]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for CopyBufferInfo2Builder<'a> {
    type Target = vk::CopyBufferInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CopyImageInfo2 {
    type Type = CopyImageInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyImageInfo2Builder<'a> {
    inner: vk::CopyImageInfo2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CopyImageInfo2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_image(mut self, src_image: vk::Image) -> Self {
        self.inner.src_image = Some(src_image);
        self
    }
    pub fn src_image_layout(mut self, src_image_layout: vk::ImageLayout) -> Self {
        self.inner.src_image_layout = src_image_layout;
        self
    }
    pub fn dst_image(mut self, dst_image: vk::Image) -> Self {
        self.inner.dst_image = Some(dst_image);
        self
    }
    pub fn dst_image_layout(mut self, dst_image_layout: vk::ImageLayout) -> Self {
        self.inner.dst_image_layout = dst_image_layout;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::ImageCopy2]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for CopyImageInfo2Builder<'a> {
    type Target = vk::CopyImageInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BlitImageInfo2 {
    type Type = BlitImageInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BlitImageInfo2Builder<'a> {
    inner: vk::BlitImageInfo2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BlitImageInfo2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_image(mut self, src_image: vk::Image) -> Self {
        self.inner.src_image = Some(src_image);
        self
    }
    pub fn src_image_layout(mut self, src_image_layout: vk::ImageLayout) -> Self {
        self.inner.src_image_layout = src_image_layout;
        self
    }
    pub fn dst_image(mut self, dst_image: vk::Image) -> Self {
        self.inner.dst_image = Some(dst_image);
        self
    }
    pub fn dst_image_layout(mut self, dst_image_layout: vk::ImageLayout) -> Self {
        self.inner.dst_image_layout = dst_image_layout;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::ImageBlit2]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn filter(mut self, filter: vk::Filter) -> Self {
        self.inner.filter = filter;
        self
    }
}
impl<'a> Deref for BlitImageInfo2Builder<'a> {
    type Target = vk::BlitImageInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CopyBufferToImageInfo2 {
    type Type = CopyBufferToImageInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyBufferToImageInfo2Builder<'a> {
    inner: vk::CopyBufferToImageInfo2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CopyBufferToImageInfo2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_buffer(mut self, src_buffer: vk::Buffer) -> Self {
        self.inner.src_buffer = Some(src_buffer);
        self
    }
    pub fn dst_image(mut self, dst_image: vk::Image) -> Self {
        self.inner.dst_image = Some(dst_image);
        self
    }
    pub fn dst_image_layout(mut self, dst_image_layout: vk::ImageLayout) -> Self {
        self.inner.dst_image_layout = dst_image_layout;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::BufferImageCopy2]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for CopyBufferToImageInfo2Builder<'a> {
    type Target = vk::CopyBufferToImageInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CopyImageToBufferInfo2 {
    type Type = CopyImageToBufferInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyImageToBufferInfo2Builder<'a> {
    inner: vk::CopyImageToBufferInfo2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CopyImageToBufferInfo2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_image(mut self, src_image: vk::Image) -> Self {
        self.inner.src_image = Some(src_image);
        self
    }
    pub fn src_image_layout(mut self, src_image_layout: vk::ImageLayout) -> Self {
        self.inner.src_image_layout = src_image_layout;
        self
    }
    pub fn dst_buffer(mut self, dst_buffer: vk::Buffer) -> Self {
        self.inner.dst_buffer = Some(dst_buffer);
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::BufferImageCopy2]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for CopyImageToBufferInfo2Builder<'a> {
    type Target = vk::CopyImageToBufferInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ResolveImageInfo2 {
    type Type = ResolveImageInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ResolveImageInfo2Builder<'a> {
    inner: vk::ResolveImageInfo2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ResolveImageInfo2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_image(mut self, src_image: vk::Image) -> Self {
        self.inner.src_image = Some(src_image);
        self
    }
    pub fn src_image_layout(mut self, src_image_layout: vk::ImageLayout) -> Self {
        self.inner.src_image_layout = src_image_layout;
        self
    }
    pub fn dst_image(mut self, dst_image: vk::Image) -> Self {
        self.inner.dst_image = Some(dst_image);
        self
    }
    pub fn dst_image_layout(mut self, dst_image_layout: vk::ImageLayout) -> Self {
        self.inner.dst_image_layout = dst_image_layout;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::ImageResolve2]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for ResolveImageInfo2Builder<'a> {
    type Target = vk::ResolveImageInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    type Type = PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder {
    inner: vk::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT,
}
impl PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_image_int64_atomics(mut self, shader_image_int64_atomics: bool) -> Self {
        self.inner.shader_image_int64_atomics = if shader_image_int64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn sparse_image_int64_atomics(mut self, sparse_image_int64_atomics: bool) -> Self {
        self.inner.sparse_image_int64_atomics = if sparse_image_int64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {}
impl<'a> Builder<'a> for vk::FragmentShadingRateAttachmentInfoKHR {
    type Type = FragmentShadingRateAttachmentInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    inner: vk::FragmentShadingRateAttachmentInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_fragment_shading_rate_attachment(
        mut self,
        p_fragment_shading_rate_attachment: Option<&'a vk::AttachmentReference2>,
    ) -> Self {
        self.inner.p_fragment_shading_rate_attachment = p_fragment_shading_rate_attachment.map_or(ptr::null(), |p| p);
        self
    }
    pub fn shading_rate_attachment_texel_size(mut self, shading_rate_attachment_texel_size: vk::Extent2D) -> Self {
        self.inner.shading_rate_attachment_texel_size = shading_rate_attachment_texel_size;
        self
    }
}
impl<'a> Deref for FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    type Target = vk::FragmentShadingRateAttachmentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> SubpassDescription2Next for FragmentShadingRateAttachmentInfoKHRBuilder<'a> {}
impl SubpassDescription2Next for vk::FragmentShadingRateAttachmentInfoKHR {}
impl Builder<'_> for vk::PipelineFragmentShadingRateStateCreateInfoKHR {
    type Type = PipelineFragmentShadingRateStateCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineFragmentShadingRateStateCreateInfoKHRBuilder {
    inner: vk::PipelineFragmentShadingRateStateCreateInfoKHR,
}
impl PipelineFragmentShadingRateStateCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_size(mut self, fragment_size: vk::Extent2D) -> Self {
        self.inner.fragment_size = fragment_size;
        self
    }
}
impl Deref for PipelineFragmentShadingRateStateCreateInfoKHRBuilder {
    type Target = vk::PipelineFragmentShadingRateStateCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl GraphicsPipelineCreateInfoNext for PipelineFragmentShadingRateStateCreateInfoKHRBuilder {}
impl GraphicsPipelineCreateInfoNext for vk::PipelineFragmentShadingRateStateCreateInfoKHR {}
impl Builder<'_> for vk::PhysicalDeviceFragmentShadingRateFeaturesKHR {
    type Type = PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder {
    inner: vk::PhysicalDeviceFragmentShadingRateFeaturesKHR,
}
impl PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline_fragment_shading_rate(mut self, pipeline_fragment_shading_rate: bool) -> Self {
        self.inner.pipeline_fragment_shading_rate = if pipeline_fragment_shading_rate {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn primitive_fragment_shading_rate(mut self, primitive_fragment_shading_rate: bool) -> Self {
        self.inner.primitive_fragment_shading_rate = if primitive_fragment_shading_rate {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn attachment_fragment_shading_rate(mut self, attachment_fragment_shading_rate: bool) -> Self {
        self.inner.attachment_fragment_shading_rate = if attachment_fragment_shading_rate {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder {
    type Target = vk::PhysicalDeviceFragmentShadingRateFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFragmentShadingRateFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFragmentShadingRateFeaturesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceFragmentShadingRatePropertiesKHR {}
impl Builder<'_> for vk::PhysicalDeviceShaderTerminateInvocationFeatures {
    type Type = PhysicalDeviceShaderTerminateInvocationFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderTerminateInvocationFeaturesBuilder {
    inner: vk::PhysicalDeviceShaderTerminateInvocationFeatures,
}
impl PhysicalDeviceShaderTerminateInvocationFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_terminate_invocation(mut self, shader_terminate_invocation: bool) -> Self {
        self.inner.shader_terminate_invocation = if shader_terminate_invocation {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShaderTerminateInvocationFeaturesBuilder {
    type Target = vk::PhysicalDeviceShaderTerminateInvocationFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceShaderTerminateInvocationFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceShaderTerminateInvocationFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderTerminateInvocationFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderTerminateInvocationFeatures {}
impl Builder<'_> for vk::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    type Type = PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder {
    inner: vk::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV,
}
impl PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_shading_rate_enums(mut self, fragment_shading_rate_enums: bool) -> Self {
        self.inner.fragment_shading_rate_enums = if fragment_shading_rate_enums {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn supersample_fragment_shading_rates(mut self, supersample_fragment_shading_rates: bool) -> Self {
        self.inner.supersample_fragment_shading_rates = if supersample_fragment_shading_rates {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn no_invocation_fragment_shading_rates(mut self, no_invocation_fragment_shading_rates: bool) -> Self {
        self.inner.no_invocation_fragment_shading_rates = if no_invocation_fragment_shading_rates {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {}
impl Builder<'_> for vk::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    type Type = PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder {
    inner: vk::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV,
}
impl PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn max_fragment_shading_rate_invocation_count(
        mut self,
        max_fragment_shading_rate_invocation_count: vk::SampleCountFlags,
    ) -> Self {
        self.inner.max_fragment_shading_rate_invocation_count = max_fragment_shading_rate_invocation_count;
        self
    }
}
impl Deref for PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder {
    type Target = vk::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceProperties2Next for PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {}
impl Builder<'_> for vk::PipelineFragmentShadingRateEnumStateCreateInfoNV {
    type Type = PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder {
    inner: vk::PipelineFragmentShadingRateEnumStateCreateInfoNV,
}
impl PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shading_rate_type(mut self, shading_rate_type: vk::FragmentShadingRateTypeNV) -> Self {
        self.inner.shading_rate_type = shading_rate_type;
        self
    }
    pub fn shading_rate(mut self, shading_rate: vk::FragmentShadingRateNV) -> Self {
        self.inner.shading_rate = shading_rate;
        self
    }
}
impl Deref for PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder {
    type Target = vk::PipelineFragmentShadingRateEnumStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl GraphicsPipelineCreateInfoNext for PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder {}
impl GraphicsPipelineCreateInfoNext for vk::PipelineFragmentShadingRateEnumStateCreateInfoNV {}
impl Builder<'_> for vk::AccelerationStructureBuildSizesInfoKHR {
    type Type = AccelerationStructureBuildSizesInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureBuildSizesInfoKHRBuilder {
    inner: vk::AccelerationStructureBuildSizesInfoKHR,
}
impl AccelerationStructureBuildSizesInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn acceleration_structure_size(mut self, acceleration_structure_size: vk::DeviceSize) -> Self {
        self.inner.acceleration_structure_size = acceleration_structure_size;
        self
    }
    pub fn update_scratch_size(mut self, update_scratch_size: vk::DeviceSize) -> Self {
        self.inner.update_scratch_size = update_scratch_size;
        self
    }
    pub fn build_scratch_size(mut self, build_scratch_size: vk::DeviceSize) -> Self {
        self.inner.build_scratch_size = build_scratch_size;
        self
    }
}
impl Deref for AccelerationStructureBuildSizesInfoKHRBuilder {
    type Target = vk::AccelerationStructureBuildSizesInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    type Type = PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder {
    inner: vk::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE,
}
impl PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn mutable_descriptor_type(mut self, mutable_descriptor_type: bool) -> Self {
        self.inner.mutable_descriptor_type = if mutable_descriptor_type { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder {
    type Target = vk::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceMutableDescriptorTypeFeaturesVALVEBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {}
impl<'a> Builder<'a> for vk::MutableDescriptorTypeListVALVE {
    type Type = MutableDescriptorTypeListVALVEBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MutableDescriptorTypeListVALVEBuilder<'a> {
    inner: vk::MutableDescriptorTypeListVALVE,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> MutableDescriptorTypeListVALVEBuilder<'a> {
    pub fn p_descriptor_types(mut self, p_descriptor_types: &'a [vk::DescriptorType]) -> Self {
        self.inner.descriptor_type_count = p_descriptor_types.len() as u32;
        self.inner.p_descriptor_types = p_descriptor_types.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for MutableDescriptorTypeListVALVEBuilder<'a> {
    type Target = vk::MutableDescriptorTypeListVALVE;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::MutableDescriptorTypeCreateInfoVALVE {
    type Type = MutableDescriptorTypeCreateInfoVALVEBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MutableDescriptorTypeCreateInfoVALVEBuilder<'a> {
    inner: vk::MutableDescriptorTypeCreateInfoVALVE,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> MutableDescriptorTypeCreateInfoVALVEBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_mutable_descriptor_type_lists(
        mut self,
        p_mutable_descriptor_type_lists: &'a [vk::MutableDescriptorTypeListVALVE],
    ) -> Self {
        self.inner.mutable_descriptor_type_list_count = p_mutable_descriptor_type_lists.len() as u32;
        self.inner.p_mutable_descriptor_type_lists = p_mutable_descriptor_type_lists
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for MutableDescriptorTypeCreateInfoVALVEBuilder<'a> {
    type Target = vk::MutableDescriptorTypeCreateInfoVALVE;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> DescriptorSetLayoutCreateInfoNext for MutableDescriptorTypeCreateInfoVALVEBuilder<'a> {}
impl<'a> DescriptorPoolCreateInfoNext for MutableDescriptorTypeCreateInfoVALVEBuilder<'a> {}
impl DescriptorSetLayoutCreateInfoNext for vk::MutableDescriptorTypeCreateInfoVALVE {}
impl DescriptorPoolCreateInfoNext for vk::MutableDescriptorTypeCreateInfoVALVE {}
impl Builder<'_> for vk::PhysicalDeviceDepthClipControlFeaturesEXT {
    type Type = PhysicalDeviceDepthClipControlFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDepthClipControlFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceDepthClipControlFeaturesEXT,
}
impl PhysicalDeviceDepthClipControlFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn depth_clip_control(mut self, depth_clip_control: bool) -> Self {
        self.inner.depth_clip_control = if depth_clip_control { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceDepthClipControlFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceDepthClipControlFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceDepthClipControlFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceDepthClipControlFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDepthClipControlFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDepthClipControlFeaturesEXT {}
impl Builder<'_> for vk::PipelineViewportDepthClipControlCreateInfoEXT {
    type Type = PipelineViewportDepthClipControlCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportDepthClipControlCreateInfoEXTBuilder {
    inner: vk::PipelineViewportDepthClipControlCreateInfoEXT,
}
impl PipelineViewportDepthClipControlCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn negative_one_to_one(mut self, negative_one_to_one: bool) -> Self {
        self.inner.negative_one_to_one = if negative_one_to_one { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PipelineViewportDepthClipControlCreateInfoEXTBuilder {
    type Target = vk::PipelineViewportDepthClipControlCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineViewportStateCreateInfoNext for PipelineViewportDepthClipControlCreateInfoEXTBuilder {}
impl PipelineViewportStateCreateInfoNext for vk::PipelineViewportDepthClipControlCreateInfoEXT {}
impl Builder<'_> for vk::PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    type Type = PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceVertexInputDynamicStateFeaturesEXT,
}
impl PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn vertex_input_dynamic_state(mut self, vertex_input_dynamic_state: bool) -> Self {
        self.inner.vertex_input_dynamic_state = if vertex_input_dynamic_state {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceVertexInputDynamicStateFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceVertexInputDynamicStateFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceVertexInputDynamicStateFeaturesEXT {}
impl Builder<'_> for vk::PhysicalDeviceExternalMemoryRDMAFeaturesNV {
    type Type = PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder {
    inner: vk::PhysicalDeviceExternalMemoryRDMAFeaturesNV,
}
impl PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn external_memory_rdma(mut self, external_memory_rdma: bool) -> Self {
        self.inner.external_memory_rdma = if external_memory_rdma { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceExternalMemoryRDMAFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceExternalMemoryRDMAFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceExternalMemoryRDMAFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceExternalMemoryRDMAFeaturesNV {}
impl Builder<'_> for vk::VertexInputBindingDescription2EXT {
    type Type = VertexInputBindingDescription2EXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct VertexInputBindingDescription2EXTBuilder {
    inner: vk::VertexInputBindingDescription2EXT,
}
impl VertexInputBindingDescription2EXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn binding(mut self, binding: u32) -> Self {
        self.inner.binding = binding;
        self
    }
    pub fn stride(mut self, stride: u32) -> Self {
        self.inner.stride = stride;
        self
    }
    pub fn input_rate(mut self, input_rate: vk::VertexInputRate) -> Self {
        self.inner.input_rate = input_rate;
        self
    }
    pub fn divisor(mut self, divisor: u32) -> Self {
        self.inner.divisor = divisor;
        self
    }
}
impl Deref for VertexInputBindingDescription2EXTBuilder {
    type Target = vk::VertexInputBindingDescription2EXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::VertexInputAttributeDescription2EXT {
    type Type = VertexInputAttributeDescription2EXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct VertexInputAttributeDescription2EXTBuilder {
    inner: vk::VertexInputAttributeDescription2EXT,
}
impl VertexInputAttributeDescription2EXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn location(mut self, location: u32) -> Self {
        self.inner.location = location;
        self
    }
    pub fn binding(mut self, binding: u32) -> Self {
        self.inner.binding = binding;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }
}
impl Deref for VertexInputAttributeDescription2EXTBuilder {
    type Target = vk::VertexInputAttributeDescription2EXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceColorWriteEnableFeaturesEXT {
    type Type = PhysicalDeviceColorWriteEnableFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceColorWriteEnableFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceColorWriteEnableFeaturesEXT,
}
impl PhysicalDeviceColorWriteEnableFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn color_write_enable(mut self, color_write_enable: bool) -> Self {
        self.inner.color_write_enable = if color_write_enable { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceColorWriteEnableFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceColorWriteEnableFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceColorWriteEnableFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceColorWriteEnableFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceColorWriteEnableFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceColorWriteEnableFeaturesEXT {}
impl<'a> Builder<'a> for vk::PipelineColorWriteCreateInfoEXT {
    type Type = PipelineColorWriteCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineColorWriteCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineColorWriteCreateInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineColorWriteCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_color_write_enables(mut self, p_color_write_enables: &'a [vk::Bool32]) -> Self {
        self.inner.attachment_count = p_color_write_enables.len() as u32;
        self.inner.p_color_write_enables = p_color_write_enables.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for PipelineColorWriteCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineColorWriteCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> PipelineColorBlendStateCreateInfoNext for PipelineColorWriteCreateInfoEXTBuilder<'a> {}
impl PipelineColorBlendStateCreateInfoNext for vk::PipelineColorWriteCreateInfoEXT {}
impl Builder<'_> for vk::MemoryBarrier2 {
    type Type = MemoryBarrier2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryBarrier2Builder {
    inner: vk::MemoryBarrier2,
}
impl MemoryBarrier2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_stage_mask(mut self, src_stage_mask: vk::PipelineStageFlags2) -> Self {
        self.inner.src_stage_mask = src_stage_mask;
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: vk::AccessFlags2) -> Self {
        self.inner.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_stage_mask(mut self, dst_stage_mask: vk::PipelineStageFlags2) -> Self {
        self.inner.dst_stage_mask = dst_stage_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: vk::AccessFlags2) -> Self {
        self.inner.dst_access_mask = dst_access_mask;
        self
    }
}
impl Deref for MemoryBarrier2Builder {
    type Target = vk::MemoryBarrier2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubpassDependency2Next for MemoryBarrier2Builder {}
impl SubpassDependency2Next for vk::MemoryBarrier2 {}
impl<'a> Builder<'a> for vk::ImageMemoryBarrier2 {
    type Type = ImageMemoryBarrier2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ImageMemoryBarrier2Next {}
#[derive(Default)]
pub struct ImageMemoryBarrier2Builder<'a> {
    inner: vk::ImageMemoryBarrier2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ImageMemoryBarrier2Builder<'a> {
    pub fn insert_next<T: ImageMemoryBarrier2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_stage_mask(mut self, src_stage_mask: vk::PipelineStageFlags2) -> Self {
        self.inner.src_stage_mask = src_stage_mask;
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: vk::AccessFlags2) -> Self {
        self.inner.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_stage_mask(mut self, dst_stage_mask: vk::PipelineStageFlags2) -> Self {
        self.inner.dst_stage_mask = dst_stage_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: vk::AccessFlags2) -> Self {
        self.inner.dst_access_mask = dst_access_mask;
        self
    }
    pub fn old_layout(mut self, old_layout: vk::ImageLayout) -> Self {
        self.inner.old_layout = old_layout;
        self
    }
    pub fn new_layout(mut self, new_layout: vk::ImageLayout) -> Self {
        self.inner.new_layout = new_layout;
        self
    }
    pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.inner.src_queue_family_index = src_queue_family_index;
        self
    }
    pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.inner.dst_queue_family_index = dst_queue_family_index;
        self
    }
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
    pub fn subresource_range(mut self, subresource_range: vk::ImageSubresourceRange) -> Self {
        self.inner.subresource_range = subresource_range;
        self
    }
}
impl<'a> Deref for ImageMemoryBarrier2Builder<'a> {
    type Target = vk::ImageMemoryBarrier2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BufferMemoryBarrier2 {
    type Type = BufferMemoryBarrier2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferMemoryBarrier2Builder {
    inner: vk::BufferMemoryBarrier2,
}
impl BufferMemoryBarrier2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_stage_mask(mut self, src_stage_mask: vk::PipelineStageFlags2) -> Self {
        self.inner.src_stage_mask = src_stage_mask;
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: vk::AccessFlags2) -> Self {
        self.inner.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_stage_mask(mut self, dst_stage_mask: vk::PipelineStageFlags2) -> Self {
        self.inner.dst_stage_mask = dst_stage_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: vk::AccessFlags2) -> Self {
        self.inner.dst_access_mask = dst_access_mask;
        self
    }
    pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.inner.src_queue_family_index = src_queue_family_index;
        self
    }
    pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.inner.dst_queue_family_index = dst_queue_family_index;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
}
impl Deref for BufferMemoryBarrier2Builder {
    type Target = vk::BufferMemoryBarrier2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DependencyInfo {
    type Type = DependencyInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DependencyInfoBuilder<'a> {
    inner: vk::DependencyInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DependencyInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dependency_flags(mut self, dependency_flags: vk::DependencyFlags) -> Self {
        self.inner.dependency_flags = dependency_flags;
        self
    }
    pub fn p_memory_barriers(mut self, p_memory_barriers: &'a [vk::MemoryBarrier2]) -> Self {
        self.inner.memory_barrier_count = p_memory_barriers.len() as u32;
        self.inner.p_memory_barriers = p_memory_barriers.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_buffer_memory_barriers(mut self, p_buffer_memory_barriers: &'a [vk::BufferMemoryBarrier2]) -> Self {
        self.inner.buffer_memory_barrier_count = p_buffer_memory_barriers.len() as u32;
        self.inner.p_buffer_memory_barriers = p_buffer_memory_barriers.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_image_memory_barriers(mut self, p_image_memory_barriers: &'a [vk::ImageMemoryBarrier2]) -> Self {
        self.inner.image_memory_barrier_count = p_image_memory_barriers.len() as u32;
        self.inner.p_image_memory_barriers = p_image_memory_barriers.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for DependencyInfoBuilder<'a> {
    type Target = vk::DependencyInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SemaphoreSubmitInfo {
    type Type = SemaphoreSubmitInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreSubmitInfoBuilder {
    inner: vk::SemaphoreSubmitInfo,
}
impl SemaphoreSubmitInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn value(mut self, value: u64) -> Self {
        self.inner.value = value;
        self
    }
    pub fn stage_mask(mut self, stage_mask: vk::PipelineStageFlags2) -> Self {
        self.inner.stage_mask = stage_mask;
        self
    }
    pub fn device_index(mut self, device_index: u32) -> Self {
        self.inner.device_index = device_index;
        self
    }
}
impl Deref for SemaphoreSubmitInfoBuilder {
    type Target = vk::SemaphoreSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CommandBufferSubmitInfo {
    type Type = CommandBufferSubmitInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandBufferSubmitInfoBuilder {
    inner: vk::CommandBufferSubmitInfo,
}
impl CommandBufferSubmitInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn command_buffer(mut self, command_buffer: vk::CommandBuffer) -> Self {
        self.inner.command_buffer = Some(command_buffer);
        self
    }
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.inner.device_mask = device_mask;
        self
    }
}
impl Deref for CommandBufferSubmitInfoBuilder {
    type Target = vk::CommandBufferSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SubmitInfo2 {
    type Type = SubmitInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SubmitInfo2Next {}
#[derive(Default)]
pub struct SubmitInfo2Builder<'a> {
    inner: vk::SubmitInfo2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SubmitInfo2Builder<'a> {
    pub fn insert_next<T: SubmitInfo2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SubmitFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_wait_semaphore_infos(mut self, p_wait_semaphore_infos: &'a [vk::SemaphoreSubmitInfo]) -> Self {
        self.inner.wait_semaphore_info_count = p_wait_semaphore_infos.len() as u32;
        self.inner.p_wait_semaphore_infos = p_wait_semaphore_infos.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_command_buffer_infos(mut self, p_command_buffer_infos: &'a [vk::CommandBufferSubmitInfo]) -> Self {
        self.inner.command_buffer_info_count = p_command_buffer_infos.len() as u32;
        self.inner.p_command_buffer_infos = p_command_buffer_infos.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_signal_semaphore_infos(mut self, p_signal_semaphore_infos: &'a [vk::SemaphoreSubmitInfo]) -> Self {
        self.inner.signal_semaphore_info_count = p_signal_semaphore_infos.len() as u32;
        self.inner.p_signal_semaphore_infos = p_signal_semaphore_infos.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for SubmitInfo2Builder<'a> {
    type Target = vk::SubmitInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl QueueFamilyProperties2Next for vk::QueueFamilyCheckpointProperties2NV {}
impl Builder<'_> for vk::PhysicalDeviceSynchronization2Features {
    type Type = PhysicalDeviceSynchronization2FeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceSynchronization2FeaturesBuilder {
    inner: vk::PhysicalDeviceSynchronization2Features,
}
impl PhysicalDeviceSynchronization2FeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn synchronization2(mut self, synchronization2: bool) -> Self {
        self.inner.synchronization2 = if synchronization2 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceSynchronization2FeaturesBuilder {
    type Target = vk::PhysicalDeviceSynchronization2Features;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceSynchronization2FeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceSynchronization2FeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceSynchronization2Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceSynchronization2Features {}
impl Builder<'_> for vk::PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
    type Type = PhysicalDevicePrimitivesGeneratedQueryFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePrimitivesGeneratedQueryFeaturesEXTBuilder {
    inner: vk::PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT,
}
impl PhysicalDevicePrimitivesGeneratedQueryFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn primitives_generated_query(mut self, primitives_generated_query: bool) -> Self {
        self.inner.primitives_generated_query = if primitives_generated_query {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn primitives_generated_query_with_rasterizer_discard(
        mut self,
        primitives_generated_query_with_rasterizer_discard: bool,
    ) -> Self {
        self.inner.primitives_generated_query_with_rasterizer_discard =
            if primitives_generated_query_with_rasterizer_discard {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn primitives_generated_query_with_non_zero_streams(
        mut self,
        primitives_generated_query_with_non_zero_streams: bool,
    ) -> Self {
        self.inner.primitives_generated_query_with_non_zero_streams =
            if primitives_generated_query_with_non_zero_streams {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
}
impl Deref for PhysicalDevicePrimitivesGeneratedQueryFeaturesEXTBuilder {
    type Target = vk::PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDevicePrimitivesGeneratedQueryFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDevicePrimitivesGeneratedQueryFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {}
impl Builder<'_> for vk::PhysicalDeviceInheritedViewportScissorFeaturesNV {
    type Type = PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder {
    inner: vk::PhysicalDeviceInheritedViewportScissorFeaturesNV,
}
impl PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn inherited_viewport_scissor_2d(mut self, inherited_viewport_scissor_2d: bool) -> Self {
        self.inner.inherited_viewport_scissor_2d = if inherited_viewport_scissor_2d {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceInheritedViewportScissorFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceInheritedViewportScissorFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceInheritedViewportScissorFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceInheritedViewportScissorFeaturesNV {}
impl<'a> Builder<'a> for vk::CommandBufferInheritanceViewportScissorInfoNV {
    type Type = CommandBufferInheritanceViewportScissorInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandBufferInheritanceViewportScissorInfoNVBuilder<'a> {
    inner: vk::CommandBufferInheritanceViewportScissorInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CommandBufferInheritanceViewportScissorInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn viewport_scissor_2d(mut self, viewport_scissor_2d: bool) -> Self {
        self.inner.viewport_scissor_2d = if viewport_scissor_2d { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn viewport_depth_count(mut self, viewport_depth_count: u32) -> Self {
        self.inner.viewport_depth_count = viewport_depth_count;
        self
    }
    pub fn p_viewport_depths(mut self, p_viewport_depths: &'a vk::Viewport) -> Self {
        self.inner.p_viewport_depths = p_viewport_depths;
        self
    }
}
impl<'a> Deref for CommandBufferInheritanceViewportScissorInfoNVBuilder<'a> {
    type Target = vk::CommandBufferInheritanceViewportScissorInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> CommandBufferInheritanceInfoNext for CommandBufferInheritanceViewportScissorInfoNVBuilder<'a> {}
impl CommandBufferInheritanceInfoNext for vk::CommandBufferInheritanceViewportScissorInfoNV {}
impl Builder<'_> for vk::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    type Type = PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT,
}
impl PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ycbcr2plane444_formats(mut self, ycbcr2plane444_formats: bool) -> Self {
        self.inner.ycbcr2plane444_formats = if ycbcr2plane444_formats { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {}
impl Builder<'_> for vk::PhysicalDeviceProvokingVertexFeaturesEXT {
    type Type = PhysicalDeviceProvokingVertexFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceProvokingVertexFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceProvokingVertexFeaturesEXT,
}
impl PhysicalDeviceProvokingVertexFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn provoking_vertex_last(mut self, provoking_vertex_last: bool) -> Self {
        self.inner.provoking_vertex_last = if provoking_vertex_last { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn transform_feedback_preserves_provoking_vertex(
        mut self,
        transform_feedback_preserves_provoking_vertex: bool,
    ) -> Self {
        self.inner.transform_feedback_preserves_provoking_vertex = if transform_feedback_preserves_provoking_vertex {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceProvokingVertexFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceProvokingVertexFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceProvokingVertexFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceProvokingVertexFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceProvokingVertexFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceProvokingVertexFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceProvokingVertexPropertiesEXT {}
impl Builder<'_> for vk::PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    type Type = PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder {
    inner: vk::PipelineRasterizationProvokingVertexStateCreateInfoEXT,
}
impl PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn provoking_vertex_mode(mut self, provoking_vertex_mode: vk::ProvokingVertexModeEXT) -> Self {
        self.inner.provoking_vertex_mode = provoking_vertex_mode;
        self
    }
}
impl Deref for PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder {
    type Target = vk::PipelineRasterizationProvokingVertexStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineRasterizationStateCreateInfoNext for PipelineRasterizationProvokingVertexStateCreateInfoEXTBuilder {}
impl PipelineRasterizationStateCreateInfoNext for vk::PipelineRasterizationProvokingVertexStateCreateInfoEXT {}
impl<'a> Builder<'a> for vk::CuModuleCreateInfoNVX {
    type Type = CuModuleCreateInfoNVXBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CuModuleCreateInfoNVXBuilder<'a> {
    inner: vk::CuModuleCreateInfoNVX,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CuModuleCreateInfoNVXBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_data<T>(mut self, p_data: &'a [T]) -> Self {
        self.inner.data_size = mem::size_of_val(p_data) as usize;
        self.inner.p_data = p_data.first().map_or(ptr::null(), |s| s as *const _) as *const _;
        self
    }
}
impl<'a> Deref for CuModuleCreateInfoNVXBuilder<'a> {
    type Target = vk::CuModuleCreateInfoNVX;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CuFunctionCreateInfoNVX {
    type Type = CuFunctionCreateInfoNVXBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CuFunctionCreateInfoNVXBuilder<'a> {
    inner: vk::CuFunctionCreateInfoNVX,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CuFunctionCreateInfoNVXBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn module(mut self, module: vk::CuModuleNVX) -> Self {
        self.inner.module = Some(module);
        self
    }
    pub fn p_name(mut self, p_name: &'a CStr) -> Self {
        self.inner.p_name = p_name.as_ptr();
        self
    }
}
impl<'a> Deref for CuFunctionCreateInfoNVXBuilder<'a> {
    type Target = vk::CuFunctionCreateInfoNVX;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CuLaunchInfoNVX {
    type Type = CuLaunchInfoNVXBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CuLaunchInfoNVXBuilder<'a> {
    inner: vk::CuLaunchInfoNVX,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CuLaunchInfoNVXBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn function(mut self, function: vk::CuFunctionNVX) -> Self {
        self.inner.function = Some(function);
        self
    }
    pub fn grid_dim_x(mut self, grid_dim_x: u32) -> Self {
        self.inner.grid_dim_x = grid_dim_x;
        self
    }
    pub fn grid_dim_y(mut self, grid_dim_y: u32) -> Self {
        self.inner.grid_dim_y = grid_dim_y;
        self
    }
    pub fn grid_dim_z(mut self, grid_dim_z: u32) -> Self {
        self.inner.grid_dim_z = grid_dim_z;
        self
    }
    pub fn block_dim_x(mut self, block_dim_x: u32) -> Self {
        self.inner.block_dim_x = block_dim_x;
        self
    }
    pub fn block_dim_y(mut self, block_dim_y: u32) -> Self {
        self.inner.block_dim_y = block_dim_y;
        self
    }
    pub fn block_dim_z(mut self, block_dim_z: u32) -> Self {
        self.inner.block_dim_z = block_dim_z;
        self
    }
    pub fn shared_mem_bytes(mut self, shared_mem_bytes: u32) -> Self {
        self.inner.shared_mem_bytes = shared_mem_bytes;
        self
    }
    pub fn p_params(mut self, p_params: &'a [*const c_void]) -> Self {
        self.inner.param_count = p_params.len() as usize;
        self.inner.p_params = p_params.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_extras(mut self, p_extras: &'a [*const c_void]) -> Self {
        self.inner.extra_count = p_extras.len() as usize;
        self.inner.p_extras = p_extras.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for CuLaunchInfoNVXBuilder<'a> {
    type Target = vk::CuLaunchInfoNVX;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderIntegerDotProductFeatures {
    type Type = PhysicalDeviceShaderIntegerDotProductFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderIntegerDotProductFeaturesBuilder {
    inner: vk::PhysicalDeviceShaderIntegerDotProductFeatures,
}
impl PhysicalDeviceShaderIntegerDotProductFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_integer_dot_product(mut self, shader_integer_dot_product: bool) -> Self {
        self.inner.shader_integer_dot_product = if shader_integer_dot_product {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShaderIntegerDotProductFeaturesBuilder {
    type Target = vk::PhysicalDeviceShaderIntegerDotProductFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceShaderIntegerDotProductFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceShaderIntegerDotProductFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderIntegerDotProductFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderIntegerDotProductFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceShaderIntegerDotProductProperties {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDrmPropertiesEXT {}
impl Builder<'_> for vk::PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    type Type = PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder {
    inner: vk::PhysicalDeviceRayTracingMotionBlurFeaturesNV,
}
impl PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ray_tracing_motion_blur(mut self, ray_tracing_motion_blur: bool) -> Self {
        self.inner.ray_tracing_motion_blur = if ray_tracing_motion_blur { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn ray_tracing_motion_blur_pipeline_trace_rays_indirect(
        mut self,
        ray_tracing_motion_blur_pipeline_trace_rays_indirect: bool,
    ) -> Self {
        self.inner.ray_tracing_motion_blur_pipeline_trace_rays_indirect =
            if ray_tracing_motion_blur_pipeline_trace_rays_indirect {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
}
impl Deref for PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceRayTracingMotionBlurFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceRayTracingMotionBlurFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRayTracingMotionBlurFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRayTracingMotionBlurFeaturesNV {}
impl Builder<'_> for vk::AccelerationStructureGeometryMotionTrianglesDataNV {
    type Type = AccelerationStructureGeometryMotionTrianglesDataNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureGeometryMotionTrianglesDataNVBuilder {
    inner: vk::AccelerationStructureGeometryMotionTrianglesDataNV,
}
impl AccelerationStructureGeometryMotionTrianglesDataNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn vertex_data(mut self, vertex_data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.vertex_data = vertex_data;
        self
    }
}
impl Deref for AccelerationStructureGeometryMotionTrianglesDataNVBuilder {
    type Target = vk::AccelerationStructureGeometryMotionTrianglesDataNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl AccelerationStructureGeometryTrianglesDataKHRNext for AccelerationStructureGeometryMotionTrianglesDataNVBuilder {}
impl AccelerationStructureGeometryTrianglesDataKHRNext for vk::AccelerationStructureGeometryMotionTrianglesDataNV {}
impl Builder<'_> for vk::AccelerationStructureMotionInfoNV {
    type Type = AccelerationStructureMotionInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureMotionInfoNVBuilder {
    inner: vk::AccelerationStructureMotionInfoNV,
}
impl AccelerationStructureMotionInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn max_instances(mut self, max_instances: u32) -> Self {
        self.inner.max_instances = max_instances;
        self
    }
    pub fn flags(mut self, flags: vk::AccelerationStructureMotionInfoFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for AccelerationStructureMotionInfoNVBuilder {
    type Target = vk::AccelerationStructureMotionInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl AccelerationStructureCreateInfoKHRNext for AccelerationStructureMotionInfoNVBuilder {}
impl AccelerationStructureCreateInfoKHRNext for vk::AccelerationStructureMotionInfoNV {}
impl Builder<'_> for vk::MemoryGetRemoteAddressInfoNV {
    type Type = MemoryGetRemoteAddressInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryGetRemoteAddressInfoNVBuilder {
    inner: vk::MemoryGetRemoteAddressInfoNV,
}
impl MemoryGetRemoteAddressInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for MemoryGetRemoteAddressInfoNVBuilder {
    type Target = vk::MemoryGetRemoteAddressInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImportMemoryBufferCollectionFUCHSIA {
    type Type = ImportMemoryBufferCollectionFUCHSIABuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportMemoryBufferCollectionFUCHSIABuilder {
    inner: vk::ImportMemoryBufferCollectionFUCHSIA,
}
impl ImportMemoryBufferCollectionFUCHSIABuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn collection(mut self, collection: vk::BufferCollectionFUCHSIA) -> Self {
        self.inner.collection = Some(collection);
        self
    }
    pub fn index(mut self, index: u32) -> Self {
        self.inner.index = index;
        self
    }
}
impl Deref for ImportMemoryBufferCollectionFUCHSIABuilder {
    type Target = vk::ImportMemoryBufferCollectionFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for ImportMemoryBufferCollectionFUCHSIABuilder {}
impl MemoryAllocateInfoNext for vk::ImportMemoryBufferCollectionFUCHSIA {}
impl Builder<'_> for vk::BufferCollectionImageCreateInfoFUCHSIA {
    type Type = BufferCollectionImageCreateInfoFUCHSIABuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferCollectionImageCreateInfoFUCHSIABuilder {
    inner: vk::BufferCollectionImageCreateInfoFUCHSIA,
}
impl BufferCollectionImageCreateInfoFUCHSIABuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn collection(mut self, collection: vk::BufferCollectionFUCHSIA) -> Self {
        self.inner.collection = Some(collection);
        self
    }
    pub fn index(mut self, index: u32) -> Self {
        self.inner.index = index;
        self
    }
}
impl Deref for BufferCollectionImageCreateInfoFUCHSIABuilder {
    type Target = vk::BufferCollectionImageCreateInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageCreateInfoNext for BufferCollectionImageCreateInfoFUCHSIABuilder {}
impl ImageCreateInfoNext for vk::BufferCollectionImageCreateInfoFUCHSIA {}
impl Builder<'_> for vk::BufferCollectionBufferCreateInfoFUCHSIA {
    type Type = BufferCollectionBufferCreateInfoFUCHSIABuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferCollectionBufferCreateInfoFUCHSIABuilder {
    inner: vk::BufferCollectionBufferCreateInfoFUCHSIA,
}
impl BufferCollectionBufferCreateInfoFUCHSIABuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn collection(mut self, collection: vk::BufferCollectionFUCHSIA) -> Self {
        self.inner.collection = Some(collection);
        self
    }
    pub fn index(mut self, index: u32) -> Self {
        self.inner.index = index;
        self
    }
}
impl Deref for BufferCollectionBufferCreateInfoFUCHSIABuilder {
    type Target = vk::BufferCollectionBufferCreateInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl BufferCreateInfoNext for BufferCollectionBufferCreateInfoFUCHSIABuilder {}
impl BufferCreateInfoNext for vk::BufferCollectionBufferCreateInfoFUCHSIA {}
impl Builder<'_> for vk::BufferCollectionCreateInfoFUCHSIA {
    type Type = BufferCollectionCreateInfoFUCHSIABuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferCollectionCreateInfoFUCHSIABuilder {
    inner: vk::BufferCollectionCreateInfoFUCHSIA,
}
impl BufferCollectionCreateInfoFUCHSIABuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn collection_token(mut self, collection_token: vk::zx_handle_t) -> Self {
        self.inner.collection_token = collection_token;
        self
    }
}
impl Deref for BufferCollectionCreateInfoFUCHSIABuilder {
    type Target = vk::BufferCollectionCreateInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BufferCollectionPropertiesFUCHSIA {
    type Type = BufferCollectionPropertiesFUCHSIABuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferCollectionPropertiesFUCHSIABuilder {
    inner: vk::BufferCollectionPropertiesFUCHSIA,
}
impl BufferCollectionPropertiesFUCHSIABuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
        self.inner.memory_type_bits = memory_type_bits;
        self
    }
    pub fn buffer_count(mut self, buffer_count: u32) -> Self {
        self.inner.buffer_count = buffer_count;
        self
    }
    pub fn create_info_index(mut self, create_info_index: u32) -> Self {
        self.inner.create_info_index = create_info_index;
        self
    }
    pub fn sysmem_pixel_format(mut self, sysmem_pixel_format: u64) -> Self {
        self.inner.sysmem_pixel_format = sysmem_pixel_format;
        self
    }
    pub fn format_features(mut self, format_features: vk::FormatFeatureFlags) -> Self {
        self.inner.format_features = format_features;
        self
    }
    pub fn sysmem_color_space_index(mut self, sysmem_color_space_index: vk::SysmemColorSpaceFUCHSIA) -> Self {
        self.inner.sysmem_color_space_index = sysmem_color_space_index;
        self
    }
    pub fn sampler_ycbcr_conversion_components(
        mut self,
        sampler_ycbcr_conversion_components: vk::ComponentMapping,
    ) -> Self {
        self.inner.sampler_ycbcr_conversion_components = sampler_ycbcr_conversion_components;
        self
    }
    pub fn suggested_ycbcr_model(mut self, suggested_ycbcr_model: vk::SamplerYcbcrModelConversion) -> Self {
        self.inner.suggested_ycbcr_model = suggested_ycbcr_model;
        self
    }
    pub fn suggested_ycbcr_range(mut self, suggested_ycbcr_range: vk::SamplerYcbcrRange) -> Self {
        self.inner.suggested_ycbcr_range = suggested_ycbcr_range;
        self
    }
    pub fn suggested_x_chroma_offset(mut self, suggested_x_chroma_offset: vk::ChromaLocation) -> Self {
        self.inner.suggested_x_chroma_offset = suggested_x_chroma_offset;
        self
    }
    pub fn suggested_y_chroma_offset(mut self, suggested_y_chroma_offset: vk::ChromaLocation) -> Self {
        self.inner.suggested_y_chroma_offset = suggested_y_chroma_offset;
        self
    }
}
impl Deref for BufferCollectionPropertiesFUCHSIABuilder {
    type Target = vk::BufferCollectionPropertiesFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BufferConstraintsInfoFUCHSIA {
    type Type = BufferConstraintsInfoFUCHSIABuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferConstraintsInfoFUCHSIABuilder {
    inner: vk::BufferConstraintsInfoFUCHSIA,
}
impl BufferConstraintsInfoFUCHSIABuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn create_info(mut self, create_info: vk::BufferCreateInfo) -> Self {
        self.inner.create_info = create_info;
        self
    }
    pub fn required_format_features(mut self, required_format_features: vk::FormatFeatureFlags) -> Self {
        self.inner.required_format_features = required_format_features;
        self
    }
    pub fn buffer_collection_constraints(
        mut self,
        buffer_collection_constraints: vk::BufferCollectionConstraintsInfoFUCHSIA,
    ) -> Self {
        self.inner.buffer_collection_constraints = buffer_collection_constraints;
        self
    }
}
impl Deref for BufferConstraintsInfoFUCHSIABuilder {
    type Target = vk::BufferConstraintsInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SysmemColorSpaceFUCHSIA {
    type Type = SysmemColorSpaceFUCHSIABuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SysmemColorSpaceFUCHSIABuilder {
    inner: vk::SysmemColorSpaceFUCHSIA,
}
impl SysmemColorSpaceFUCHSIABuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn color_space(mut self, color_space: u32) -> Self {
        self.inner.color_space = color_space;
        self
    }
}
impl Deref for SysmemColorSpaceFUCHSIABuilder {
    type Target = vk::SysmemColorSpaceFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageFormatConstraintsInfoFUCHSIA {
    type Type = ImageFormatConstraintsInfoFUCHSIABuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageFormatConstraintsInfoFUCHSIABuilder<'a> {
    inner: vk::ImageFormatConstraintsInfoFUCHSIA,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ImageFormatConstraintsInfoFUCHSIABuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image_create_info(mut self, image_create_info: vk::ImageCreateInfo) -> Self {
        self.inner.image_create_info = image_create_info;
        self
    }
    pub fn required_format_features(mut self, required_format_features: vk::FormatFeatureFlags) -> Self {
        self.inner.required_format_features = required_format_features;
        self
    }
    pub fn flags(mut self, flags: vk::ImageFormatConstraintsFlagsFUCHSIA) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn sysmem_pixel_format(mut self, sysmem_pixel_format: u64) -> Self {
        self.inner.sysmem_pixel_format = sysmem_pixel_format;
        self
    }
    pub fn p_color_spaces(mut self, p_color_spaces: &'a [vk::SysmemColorSpaceFUCHSIA]) -> Self {
        self.inner.color_space_count = p_color_spaces.len() as u32;
        self.inner.p_color_spaces = p_color_spaces.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
}
impl<'a> Deref for ImageFormatConstraintsInfoFUCHSIABuilder<'a> {
    type Target = vk::ImageFormatConstraintsInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageConstraintsInfoFUCHSIA {
    type Type = ImageConstraintsInfoFUCHSIABuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageConstraintsInfoFUCHSIABuilder<'a> {
    inner: vk::ImageConstraintsInfoFUCHSIA,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ImageConstraintsInfoFUCHSIABuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_format_constraints(mut self, p_format_constraints: &'a [vk::ImageFormatConstraintsInfoFUCHSIA]) -> Self {
        self.inner.format_constraints_count = p_format_constraints.len() as u32;
        self.inner.p_format_constraints = p_format_constraints.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn buffer_collection_constraints(
        mut self,
        buffer_collection_constraints: vk::BufferCollectionConstraintsInfoFUCHSIA,
    ) -> Self {
        self.inner.buffer_collection_constraints = buffer_collection_constraints;
        self
    }
    pub fn flags(mut self, flags: vk::ImageConstraintsInfoFlagsFUCHSIA) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl<'a> Deref for ImageConstraintsInfoFUCHSIABuilder<'a> {
    type Target = vk::ImageConstraintsInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BufferCollectionConstraintsInfoFUCHSIA {
    type Type = BufferCollectionConstraintsInfoFUCHSIABuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferCollectionConstraintsInfoFUCHSIABuilder {
    inner: vk::BufferCollectionConstraintsInfoFUCHSIA,
}
impl BufferCollectionConstraintsInfoFUCHSIABuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn min_buffer_count(mut self, min_buffer_count: u32) -> Self {
        self.inner.min_buffer_count = min_buffer_count;
        self
    }
    pub fn max_buffer_count(mut self, max_buffer_count: u32) -> Self {
        self.inner.max_buffer_count = max_buffer_count;
        self
    }
    pub fn min_buffer_count_for_camping(mut self, min_buffer_count_for_camping: u32) -> Self {
        self.inner.min_buffer_count_for_camping = min_buffer_count_for_camping;
        self
    }
    pub fn min_buffer_count_for_dedicated_slack(mut self, min_buffer_count_for_dedicated_slack: u32) -> Self {
        self.inner.min_buffer_count_for_dedicated_slack = min_buffer_count_for_dedicated_slack;
        self
    }
    pub fn min_buffer_count_for_shared_slack(mut self, min_buffer_count_for_shared_slack: u32) -> Self {
        self.inner.min_buffer_count_for_shared_slack = min_buffer_count_for_shared_slack;
        self
    }
}
impl Deref for BufferCollectionConstraintsInfoFUCHSIABuilder {
    type Target = vk::BufferCollectionConstraintsInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    type Type = PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceRGBA10X6FormatsFeaturesEXT,
}
impl PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn format_rgba10x6_without_y_cb_cr_sampler(mut self, format_rgba10x6_without_y_cb_cr_sampler: bool) -> Self {
        self.inner.format_rgba10x6_without_y_cb_cr_sampler = if format_rgba10x6_without_y_cb_cr_sampler {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceRGBA10X6FormatsFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceRGBA10X6FormatsFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRGBA10X6FormatsFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRGBA10X6FormatsFeaturesEXT {}
impl FormatProperties2Next for vk::FormatProperties3 {}
impl FormatProperties2Next for vk::DrmFormatModifierPropertiesList2EXT {}
impl AndroidHardwareBufferPropertiesANDROIDNext for vk::AndroidHardwareBufferFormatProperties2ANDROID {}
impl<'a> Builder<'a> for vk::PipelineRenderingCreateInfo {
    type Type = PipelineRenderingCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRenderingCreateInfoBuilder<'a> {
    inner: vk::PipelineRenderingCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineRenderingCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn view_mask(mut self, view_mask: u32) -> Self {
        self.inner.view_mask = view_mask;
        self
    }
    pub fn p_color_attachment_formats(mut self, p_color_attachment_formats: &'a [vk::Format]) -> Self {
        self.inner.color_attachment_count = p_color_attachment_formats.len() as u32;
        self.inner.p_color_attachment_formats = p_color_attachment_formats
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn depth_attachment_format(mut self, depth_attachment_format: vk::Format) -> Self {
        self.inner.depth_attachment_format = depth_attachment_format;
        self
    }
    pub fn stencil_attachment_format(mut self, stencil_attachment_format: vk::Format) -> Self {
        self.inner.stencil_attachment_format = stencil_attachment_format;
        self
    }
}
impl<'a> Deref for PipelineRenderingCreateInfoBuilder<'a> {
    type Target = vk::PipelineRenderingCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> GraphicsPipelineCreateInfoNext for PipelineRenderingCreateInfoBuilder<'a> {}
impl GraphicsPipelineCreateInfoNext for vk::PipelineRenderingCreateInfo {}
impl<'a> Builder<'a> for vk::RenderingInfo {
    type Type = RenderingInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait RenderingInfoNext {}
#[derive(Default)]
pub struct RenderingInfoBuilder<'a> {
    inner: vk::RenderingInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RenderingInfoBuilder<'a> {
    pub fn insert_next<T: RenderingInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::RenderingFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn render_area(mut self, render_area: vk::Rect2D) -> Self {
        self.inner.render_area = render_area;
        self
    }
    pub fn layer_count(mut self, layer_count: u32) -> Self {
        self.inner.layer_count = layer_count;
        self
    }
    pub fn view_mask(mut self, view_mask: u32) -> Self {
        self.inner.view_mask = view_mask;
        self
    }
    pub fn p_color_attachments(mut self, p_color_attachments: &'a [vk::RenderingAttachmentInfo]) -> Self {
        self.inner.color_attachment_count = p_color_attachments.len() as u32;
        self.inner.p_color_attachments = p_color_attachments.first().map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn p_depth_attachment(mut self, p_depth_attachment: Option<&'a vk::RenderingAttachmentInfo>) -> Self {
        self.inner.p_depth_attachment = p_depth_attachment.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_stencil_attachment(mut self, p_stencil_attachment: Option<&'a vk::RenderingAttachmentInfo>) -> Self {
        self.inner.p_stencil_attachment = p_stencil_attachment.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for RenderingInfoBuilder<'a> {
    type Target = vk::RenderingInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::RenderingAttachmentInfo {
    type Type = RenderingAttachmentInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderingAttachmentInfoBuilder {
    inner: vk::RenderingAttachmentInfo,
}
impl RenderingAttachmentInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image_view(mut self, image_view: Option<vk::ImageView>) -> Self {
        self.inner.image_view = image_view;
        self
    }
    pub fn image_layout(mut self, image_layout: vk::ImageLayout) -> Self {
        self.inner.image_layout = image_layout;
        self
    }
    pub fn resolve_mode(mut self, resolve_mode: vk::ResolveModeFlags) -> Self {
        self.inner.resolve_mode = resolve_mode;
        self
    }
    pub fn resolve_image_view(mut self, resolve_image_view: Option<vk::ImageView>) -> Self {
        self.inner.resolve_image_view = resolve_image_view;
        self
    }
    pub fn resolve_image_layout(mut self, resolve_image_layout: vk::ImageLayout) -> Self {
        self.inner.resolve_image_layout = resolve_image_layout;
        self
    }
    pub fn load_op(mut self, load_op: vk::AttachmentLoadOp) -> Self {
        self.inner.load_op = load_op;
        self
    }
    pub fn store_op(mut self, store_op: vk::AttachmentStoreOp) -> Self {
        self.inner.store_op = store_op;
        self
    }
    pub fn clear_value(mut self, clear_value: vk::ClearValue) -> Self {
        self.inner.clear_value = clear_value;
        self
    }
}
impl Deref for RenderingAttachmentInfoBuilder {
    type Target = vk::RenderingAttachmentInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::RenderingFragmentShadingRateAttachmentInfoKHR {
    type Type = RenderingFragmentShadingRateAttachmentInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderingFragmentShadingRateAttachmentInfoKHRBuilder {
    inner: vk::RenderingFragmentShadingRateAttachmentInfoKHR,
}
impl RenderingFragmentShadingRateAttachmentInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image_view(mut self, image_view: Option<vk::ImageView>) -> Self {
        self.inner.image_view = image_view;
        self
    }
    pub fn image_layout(mut self, image_layout: vk::ImageLayout) -> Self {
        self.inner.image_layout = image_layout;
        self
    }
    pub fn shading_rate_attachment_texel_size(mut self, shading_rate_attachment_texel_size: vk::Extent2D) -> Self {
        self.inner.shading_rate_attachment_texel_size = shading_rate_attachment_texel_size;
        self
    }
}
impl Deref for RenderingFragmentShadingRateAttachmentInfoKHRBuilder {
    type Target = vk::RenderingFragmentShadingRateAttachmentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl RenderingInfoNext for RenderingFragmentShadingRateAttachmentInfoKHRBuilder {}
impl RenderingInfoNext for vk::RenderingFragmentShadingRateAttachmentInfoKHR {}
impl Builder<'_> for vk::RenderingFragmentDensityMapAttachmentInfoEXT {
    type Type = RenderingFragmentDensityMapAttachmentInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderingFragmentDensityMapAttachmentInfoEXTBuilder {
    inner: vk::RenderingFragmentDensityMapAttachmentInfoEXT,
}
impl RenderingFragmentDensityMapAttachmentInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image_view(mut self, image_view: vk::ImageView) -> Self {
        self.inner.image_view = Some(image_view);
        self
    }
    pub fn image_layout(mut self, image_layout: vk::ImageLayout) -> Self {
        self.inner.image_layout = image_layout;
        self
    }
}
impl Deref for RenderingFragmentDensityMapAttachmentInfoEXTBuilder {
    type Target = vk::RenderingFragmentDensityMapAttachmentInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl RenderingInfoNext for RenderingFragmentDensityMapAttachmentInfoEXTBuilder {}
impl RenderingInfoNext for vk::RenderingFragmentDensityMapAttachmentInfoEXT {}
impl Builder<'_> for vk::PhysicalDeviceDynamicRenderingFeatures {
    type Type = PhysicalDeviceDynamicRenderingFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDynamicRenderingFeaturesBuilder {
    inner: vk::PhysicalDeviceDynamicRenderingFeatures,
}
impl PhysicalDeviceDynamicRenderingFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dynamic_rendering(mut self, dynamic_rendering: bool) -> Self {
        self.inner.dynamic_rendering = if dynamic_rendering { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceDynamicRenderingFeaturesBuilder {
    type Target = vk::PhysicalDeviceDynamicRenderingFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceDynamicRenderingFeaturesBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceDynamicRenderingFeaturesBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDynamicRenderingFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDynamicRenderingFeatures {}
impl<'a> Builder<'a> for vk::CommandBufferInheritanceRenderingInfo {
    type Type = CommandBufferInheritanceRenderingInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandBufferInheritanceRenderingInfoBuilder<'a> {
    inner: vk::CommandBufferInheritanceRenderingInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CommandBufferInheritanceRenderingInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::RenderingFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn view_mask(mut self, view_mask: u32) -> Self {
        self.inner.view_mask = view_mask;
        self
    }
    pub fn p_color_attachment_formats(mut self, p_color_attachment_formats: &'a [vk::Format]) -> Self {
        self.inner.color_attachment_count = p_color_attachment_formats.len() as u32;
        self.inner.p_color_attachment_formats = p_color_attachment_formats
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn depth_attachment_format(mut self, depth_attachment_format: vk::Format) -> Self {
        self.inner.depth_attachment_format = depth_attachment_format;
        self
    }
    pub fn stencil_attachment_format(mut self, stencil_attachment_format: vk::Format) -> Self {
        self.inner.stencil_attachment_format = stencil_attachment_format;
        self
    }
    pub fn rasterization_samples(mut self, rasterization_samples: vk::SampleCountFlags) -> Self {
        self.inner.rasterization_samples = rasterization_samples;
        self
    }
}
impl<'a> Deref for CommandBufferInheritanceRenderingInfoBuilder<'a> {
    type Target = vk::CommandBufferInheritanceRenderingInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> CommandBufferInheritanceInfoNext for CommandBufferInheritanceRenderingInfoBuilder<'a> {}
impl CommandBufferInheritanceInfoNext for vk::CommandBufferInheritanceRenderingInfo {}
impl<'a> Builder<'a> for vk::AttachmentSampleCountInfoAMD {
    type Type = AttachmentSampleCountInfoAMDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AttachmentSampleCountInfoAMDBuilder<'a> {
    inner: vk::AttachmentSampleCountInfoAMD,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> AttachmentSampleCountInfoAMDBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_color_attachment_samples(mut self, p_color_attachment_samples: &'a [vk::SampleCountFlags]) -> Self {
        self.inner.color_attachment_count = p_color_attachment_samples.len() as u32;
        self.inner.p_color_attachment_samples = p_color_attachment_samples
            .first()
            .map_or(ptr::null(), |s| s as *const _);
        self
    }
    pub fn depth_stencil_attachment_samples(mut self, depth_stencil_attachment_samples: vk::SampleCountFlags) -> Self {
        self.inner.depth_stencil_attachment_samples = depth_stencil_attachment_samples;
        self
    }
}
impl<'a> Deref for AttachmentSampleCountInfoAMDBuilder<'a> {
    type Target = vk::AttachmentSampleCountInfoAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> CommandBufferInheritanceInfoNext for AttachmentSampleCountInfoAMDBuilder<'a> {}
impl<'a> GraphicsPipelineCreateInfoNext for AttachmentSampleCountInfoAMDBuilder<'a> {}
impl CommandBufferInheritanceInfoNext for vk::AttachmentSampleCountInfoAMD {}
impl GraphicsPipelineCreateInfoNext for vk::AttachmentSampleCountInfoAMD {}
impl Builder<'_> for vk::MultiviewPerViewAttributesInfoNVX {
    type Type = MultiviewPerViewAttributesInfoNVXBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MultiviewPerViewAttributesInfoNVXBuilder {
    inner: vk::MultiviewPerViewAttributesInfoNVX,
}
impl MultiviewPerViewAttributesInfoNVXBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn per_view_attributes(mut self, per_view_attributes: bool) -> Self {
        self.inner.per_view_attributes = if per_view_attributes { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn per_view_attributes_position_x_only(mut self, per_view_attributes_position_x_only: bool) -> Self {
        self.inner.per_view_attributes_position_x_only = if per_view_attributes_position_x_only {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for MultiviewPerViewAttributesInfoNVXBuilder {
    type Target = vk::MultiviewPerViewAttributesInfoNVX;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl CommandBufferInheritanceInfoNext for MultiviewPerViewAttributesInfoNVXBuilder {}
impl GraphicsPipelineCreateInfoNext for MultiviewPerViewAttributesInfoNVXBuilder {}
impl RenderingInfoNext for MultiviewPerViewAttributesInfoNVXBuilder {}
impl CommandBufferInheritanceInfoNext for vk::MultiviewPerViewAttributesInfoNVX {}
impl GraphicsPipelineCreateInfoNext for vk::MultiviewPerViewAttributesInfoNVX {}
impl RenderingInfoNext for vk::MultiviewPerViewAttributesInfoNVX {}
impl Builder<'_> for vk::PhysicalDeviceImageViewMinLodFeaturesEXT {
    type Type = PhysicalDeviceImageViewMinLodFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceImageViewMinLodFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceImageViewMinLodFeaturesEXT,
}
impl PhysicalDeviceImageViewMinLodFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn min_lod(mut self, min_lod: bool) -> Self {
        self.inner.min_lod = if min_lod { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceImageViewMinLodFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceImageViewMinLodFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceImageViewMinLodFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceImageViewMinLodFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceImageViewMinLodFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceImageViewMinLodFeaturesEXT {}
impl Builder<'_> for vk::ImageViewMinLodCreateInfoEXT {
    type Type = ImageViewMinLodCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageViewMinLodCreateInfoEXTBuilder {
    inner: vk::ImageViewMinLodCreateInfoEXT,
}
impl ImageViewMinLodCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn min_lod(mut self, min_lod: f32) -> Self {
        self.inner.min_lod = min_lod;
        self
    }
}
impl Deref for ImageViewMinLodCreateInfoEXTBuilder {
    type Target = vk::ImageViewMinLodCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageViewCreateInfoNext for ImageViewMinLodCreateInfoEXTBuilder {}
impl ImageViewCreateInfoNext for vk::ImageViewMinLodCreateInfoEXT {}
impl Builder<'_> for vk::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
    type Type = PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder {
    inner: vk::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM,
}
impl PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn rasterization_order_color_attachment_access(
        mut self,
        rasterization_order_color_attachment_access: bool,
    ) -> Self {
        self.inner.rasterization_order_color_attachment_access = if rasterization_order_color_attachment_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn rasterization_order_depth_attachment_access(
        mut self,
        rasterization_order_depth_attachment_access: bool,
    ) -> Self {
        self.inner.rasterization_order_depth_attachment_access = if rasterization_order_depth_attachment_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn rasterization_order_stencil_attachment_access(
        mut self,
        rasterization_order_stencil_attachment_access: bool,
    ) -> Self {
        self.inner.rasterization_order_stencil_attachment_access = if rasterization_order_stencil_attachment_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder {
    type Target = vk::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARMBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {}
impl Builder<'_> for vk::PhysicalDeviceLinearColorAttachmentFeaturesNV {
    type Type = PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder {
    inner: vk::PhysicalDeviceLinearColorAttachmentFeaturesNV,
}
impl PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn linear_color_attachment(mut self, linear_color_attachment: bool) -> Self {
        self.inner.linear_color_attachment = if linear_color_attachment { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceLinearColorAttachmentFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceLinearColorAttachmentFeaturesNVBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceLinearColorAttachmentFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceLinearColorAttachmentFeaturesNV {}
impl Builder<'_> for vk::PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
    type Type = PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT,
}
impl PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn graphics_pipeline_library(mut self, graphics_pipeline_library: bool) -> Self {
        self.inner.graphics_pipeline_library = if graphics_pipeline_library { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXTBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {}
impl Builder<'_> for vk::PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
    type Type = PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder {
    inner: vk::PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT,
}
impl PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn graphics_pipeline_library_fast_linking(mut self, graphics_pipeline_library_fast_linking: bool) -> Self {
        self.inner.graphics_pipeline_library_fast_linking = if graphics_pipeline_library_fast_linking {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn graphics_pipeline_library_independent_interpolation_decoration(
        mut self,
        graphics_pipeline_library_independent_interpolation_decoration: bool,
    ) -> Self {
        self.inner
            .graphics_pipeline_library_independent_interpolation_decoration =
            if graphics_pipeline_library_independent_interpolation_decoration {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
}
impl Deref for PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder {
    type Target = vk::PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceProperties2Next for PhysicalDeviceGraphicsPipelineLibraryPropertiesEXTBuilder {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {}
impl Builder<'_> for vk::GraphicsPipelineLibraryCreateInfoEXT {
    type Type = GraphicsPipelineLibraryCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GraphicsPipelineLibraryCreateInfoEXTBuilder {
    inner: vk::GraphicsPipelineLibraryCreateInfoEXT,
}
impl GraphicsPipelineLibraryCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::GraphicsPipelineLibraryFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for GraphicsPipelineLibraryCreateInfoEXTBuilder {
    type Target = vk::GraphicsPipelineLibraryCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl GraphicsPipelineCreateInfoNext for GraphicsPipelineLibraryCreateInfoEXTBuilder {}
impl GraphicsPipelineCreateInfoNext for vk::GraphicsPipelineLibraryCreateInfoEXT {}
impl Builder<'_> for vk::PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
    type Type = PhysicalDeviceDescriptorSetHostMappingFeaturesVALVEBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDescriptorSetHostMappingFeaturesVALVEBuilder {
    inner: vk::PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE,
}
impl PhysicalDeviceDescriptorSetHostMappingFeaturesVALVEBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn descriptor_set_host_mapping(mut self, descriptor_set_host_mapping: bool) -> Self {
        self.inner.descriptor_set_host_mapping = if descriptor_set_host_mapping {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceDescriptorSetHostMappingFeaturesVALVEBuilder {
    type Target = vk::PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for PhysicalDeviceDescriptorSetHostMappingFeaturesVALVEBuilder {}
impl DeviceCreateInfoNext for PhysicalDeviceDescriptorSetHostMappingFeaturesVALVEBuilder {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {}
impl Builder<'_> for vk::DescriptorSetBindingReferenceVALVE {
    type Type = DescriptorSetBindingReferenceVALVEBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorSetBindingReferenceVALVEBuilder {
    inner: vk::DescriptorSetBindingReferenceVALVE,
}
impl DescriptorSetBindingReferenceVALVEBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn descriptor_set_layout(mut self, descriptor_set_layout: vk::DescriptorSetLayout) -> Self {
        self.inner.descriptor_set_layout = Some(descriptor_set_layout);
        self
    }
    pub fn binding(mut self, binding: u32) -> Self {
        self.inner.binding = binding;
        self
    }
}
impl Deref for DescriptorSetBindingReferenceVALVEBuilder {
    type Target = vk::DescriptorSetBindingReferenceVALVE;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DescriptorSetLayoutHostMappingInfoVALVE {
    type Type = DescriptorSetLayoutHostMappingInfoVALVEBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorSetLayoutHostMappingInfoVALVEBuilder {
    inner: vk::DescriptorSetLayoutHostMappingInfoVALVE,
}
impl DescriptorSetLayoutHostMappingInfoVALVEBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn descriptor_offset(mut self, descriptor_offset: usize) -> Self {
        self.inner.descriptor_offset = descriptor_offset;
        self
    }
    pub fn descriptor_size(mut self, descriptor_size: u32) -> Self {
        self.inner.descriptor_size = descriptor_size;
        self
    }
}
impl Deref for DescriptorSetLayoutHostMappingInfoVALVEBuilder {
    type Target = vk::DescriptorSetLayoutHostMappingInfoVALVE;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
