use super::{VirtFilesystem, WasiIoErrorRef};
use crate::bindings::exports::wasi::filesystem::types::{
    Advice, Descriptor, DescriptorBorrow, DescriptorFlags, DescriptorStat, DescriptorType,
    DirectoryEntry, DirectoryEntryStream, ErrorCode, Filesize, Guest as WasiFilesystemTypes,
    GuestDescriptor, GuestDirectoryEntryStream, MetadataHashValue, NewTimestamp, OpenFlags,
    PathFlags,
};
use crate::bindings::wasi::io::streams::{InputStream, OutputStream};

pub enum VirtDescriptor {}
pub enum VirtDirectoryEntryStream {}

impl WasiFilesystemTypes for VirtFilesystem {
    type Descriptor = VirtDescriptor;
    type DirectoryEntryStream = VirtDirectoryEntryStream;

    fn filesystem_error_code(_err: WasiIoErrorRef) -> Option<ErrorCode> {
        None
    }
}

#[expect(clippy::uninhabited_references)] // FIXME
impl GuestDescriptor for VirtDescriptor {
    fn read_via_stream(&self, _offset: Filesize) -> Result<InputStream, ErrorCode> {
        match *self {}
    }

    fn write_via_stream(&self, _offset: Filesize) -> Result<OutputStream, ErrorCode> {
        match *self {}
    }

    fn append_via_stream(&self) -> Result<OutputStream, ErrorCode> {
        match *self {}
    }

    fn advise(
        &self,
        _offset: Filesize,
        _length: Filesize,
        _advice: Advice,
    ) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn sync_data(&self) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn get_flags(&self) -> Result<DescriptorFlags, ErrorCode> {
        match *self {}
    }

    fn get_type(&self) -> Result<DescriptorType, ErrorCode> {
        match *self {}
    }

    fn set_size(&self, _size: Filesize) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn set_times(
        &self,
        _date_access_timestamp: NewTimestamp,
        _date_modification_timestamp: NewTimestamp,
    ) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn read(&self, _length: Filesize, _offset: Filesize) -> Result<(Vec<u8>, bool), ErrorCode> {
        match *self {}
    }

    fn write(&self, _buffer: Vec<u8>, _offset: Filesize) -> Result<u64, ErrorCode> {
        match *self {}
    }

    fn read_directory(&self) -> Result<DirectoryEntryStream, ErrorCode> {
        match *self {}
    }

    fn sync(&self) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn create_directory_at(&self, _path: String) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn stat(&self) -> Result<DescriptorStat, ErrorCode> {
        match *self {}
    }

    fn stat_at(&self, _path_flags: PathFlags, _path: String) -> Result<DescriptorStat, ErrorCode> {
        match *self {}
    }

    fn set_times_at(
        &self,
        _path_flags: PathFlags,
        _path: String,
        _date_access_timestamp: NewTimestamp,
        _date_modification_timestamp: NewTimestamp,
    ) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn link_at(
        &self,
        _old_path_flags: PathFlags,
        _old_path: String,
        _new_descriptor: DescriptorBorrow,
        _new_path: String,
    ) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn open_at(
        &self,
        _path_flags: PathFlags,
        _path: String,
        _open_flags: OpenFlags,
        _flags: DescriptorFlags,
    ) -> Result<Descriptor, ErrorCode> {
        match *self {}
    }

    fn readlink_at(&self, _path: String) -> Result<String, ErrorCode> {
        match *self {}
    }

    fn remove_directory_at(&self, _path: String) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn rename_at(
        &self,
        _old_path: String,
        _new_descriptor: DescriptorBorrow,
        _new_path: String,
    ) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn symlink_at(&self, _old_path: String, _new_path: String) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn unlink_file_at(&self, _path: String) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn is_same_object(&self, _other: DescriptorBorrow) -> bool {
        match *self {}
    }

    fn metadata_hash(&self) -> Result<MetadataHashValue, ErrorCode> {
        match *self {}
    }

    fn metadata_hash_at(
        &self,
        _path_flags: PathFlags,
        _path: String,
    ) -> Result<MetadataHashValue, ErrorCode> {
        match *self {}
    }
}

#[expect(clippy::uninhabited_references)] // FIXME
impl GuestDirectoryEntryStream for VirtDirectoryEntryStream {
    fn read_directory_entry(&self) -> Result<Option<DirectoryEntry>, ErrorCode> {
        match *self {}
    }
}
