use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

use map_vec::Map as VecMap;

use crate::bindings::exports::wasi::http::types::{
    Duration, ErrorCode, FieldName, FieldValue, Fields, FutureTrailers, Guest as WasiHttpTypes,
    GuestFields, GuestFutureIncomingResponse, GuestFutureTrailers, GuestIncomingBody,
    GuestIncomingRequest, GuestIncomingResponse, GuestOutgoingBody, GuestOutgoingRequest,
    GuestOutgoingResponse, GuestRequestOptions, GuestResponseOutparam, HeaderError, Headers,
    IncomingBody, IncomingResponse, InputStream, Method, OutgoingBody, OutgoingResponse,
    OutputStream, Pollable, ResponseOutparam, Scheme, StatusCode, Trailers,
};
use crate::bindings::wasi::null::io::output_sink;

use super::{VirtHttp, WasiIoErrorRef};

impl WasiHttpTypes for VirtHttp {
    type Fields = VirtFields;
    type IncomingRequest = VirtIncomingRequest;
    type OutgoingRequest = VirtOutgoingRequest;
    type RequestOptions = VirtRequestOptions;
    type ResponseOutparam = VirtResponseOutparam;
    type IncomingResponse = VirtIncomingResponse;
    type IncomingBody = VirtIncomingBody;
    type FutureTrailers = VirtFutureTrailers;
    type OutgoingResponse = VirtOutgoingResponse;
    type OutgoingBody = VirtOutgoingBody;
    type FutureIncomingResponse = VirtFutureIncomingResponse;

    fn http_error_code(_err: WasiIoErrorRef) -> Option<ErrorCode> {
        None
    }
}

pub struct VirtFields {
    inner: RwLock<VirtFieldsInner>,
    is_mutable: bool,
}

#[derive(Clone)]
struct VirtFieldsInner {
    fields: VecMap<FieldName, Vec<FieldValue>>,
}

impl VirtFields {
    #[expect(clippy::unwrap_used)]
    fn read(&self) -> RwLockReadGuard<'_, VirtFieldsInner> {
        self.inner.read().unwrap()
    }

    #[expect(clippy::unwrap_used)]
    fn write(&self) -> Result<RwLockWriteGuard<'_, VirtFieldsInner>, HeaderError> {
        if self.is_mutable {
            Ok(self.inner.write().unwrap())
        } else {
            Err(HeaderError::Immutable)
        }
    }
}

impl GuestFields for VirtFields {
    fn new() -> Self {
        Self {
            inner: RwLock::new(VirtFieldsInner {
                fields: VecMap::new(),
            }),
            is_mutable: true,
        }
    }

    fn from_list(entries: Vec<(FieldName, FieldValue)>) -> Result<Fields, HeaderError> {
        let mut fields: VecMap<FieldName, Vec<FieldValue>> = VecMap::new();

        for (key, value) in entries {
            fields.entry(key).or_default().push(value);
        }

        Ok(Fields::new(Self {
            inner: RwLock::new(VirtFieldsInner { fields }),
            is_mutable: true,
        }))
    }

    fn get(&self, name: FieldName) -> Vec<FieldValue> {
        self.read()
            .fields
            .get(&name)
            .map_or_else(Vec::new, Vec::clone)
    }

    fn has(&self, name: FieldName) -> bool {
        self.read().fields.contains_key(&name)
    }

    fn set(&self, name: FieldName, value: Vec<FieldValue>) -> Result<(), HeaderError> {
        self.write()?.fields.insert(name, value);
        Ok(())
    }

    fn delete(&self, name: FieldName) -> Result<(), HeaderError> {
        self.write()?.fields.remove(&name);
        Ok(())
    }

    fn append(&self, name: FieldName, value: FieldValue) -> Result<(), HeaderError> {
        self.write()?.fields.entry(name).or_default().push(value);
        Ok(())
    }

    fn entries(&self) -> Vec<(FieldName, FieldValue)> {
        self.read()
            .fields
            .iter()
            .flat_map(|(k, vs)| vs.iter().map(|v| (k.clone(), v.clone())))
            .collect()
    }

    fn clone(&self) -> Fields {
        Fields::new(Self {
            inner: RwLock::new(self.read().clone()),
            is_mutable: true,
        })
    }
}

pub enum VirtIncomingRequest {}

#[expect(clippy::uninhabited_references)] // FIXME
impl GuestIncomingRequest for VirtIncomingRequest {
    fn method(&self) -> Method {
        match *self {}
    }

    fn path_with_query(&self) -> Option<String> {
        match *self {}
    }

    fn scheme(&self) -> Option<Scheme> {
        match *self {}
    }

    fn authority(&self) -> Option<String> {
        match *self {}
    }

    fn headers(&self) -> Headers {
        match *self {}
    }

    fn consume(&self) -> Result<IncomingBody, ()> {
        match *self {}
    }
}

pub struct VirtOutgoingRequest {
    inner: RwLock<VirtOutgoingRequestInner>,
}

struct VirtOutgoingRequestInner {
    headers: Headers,
    method: Method,
    path_with_query: Option<String>,
    scheme: Option<Scheme>,
    authority: Option<String>,
    has_body: bool,
}

impl VirtOutgoingRequest {
    #[expect(clippy::unwrap_used)]
    fn read(&self) -> RwLockReadGuard<'_, VirtOutgoingRequestInner> {
        self.inner.read().unwrap()
    }

    #[expect(clippy::unwrap_used)]
    fn write(&self) -> RwLockWriteGuard<'_, VirtOutgoingRequestInner> {
        self.inner.write().unwrap()
    }
}

impl GuestOutgoingRequest for VirtOutgoingRequest {
    fn new(headers: Headers) -> Self {
        Self {
            inner: RwLock::new(VirtOutgoingRequestInner {
                headers,
                method: Method::Get,
                path_with_query: None,
                scheme: None,
                authority: None,
                has_body: true,
            }),
        }
    }

    fn body(&self) -> Result<OutgoingBody, ()> {
        let mut inner = self.write();

        if !inner.has_body {
            return Err(());
        }

        inner.has_body = false;
        std::mem::drop(inner);

        Ok(OutgoingBody::new(VirtOutgoingBody {
            inner: Mutex::new(VirtOutgoingBodyInner { has_written: false }),
        }))
    }

    fn method(&self) -> Method {
        self.read().method.clone()
    }

    fn set_method(&self, method: Method) -> Result<(), ()> {
        self.write().method = method;
        Ok(())
    }

    fn path_with_query(&self) -> Option<String> {
        self.read().path_with_query.clone()
    }

    fn set_path_with_query(&self, path_with_query: Option<String>) -> Result<(), ()> {
        self.write().path_with_query = path_with_query;
        Ok(())
    }

    fn scheme(&self) -> Option<Scheme> {
        self.read().scheme.clone()
    }

    fn set_scheme(&self, scheme: Option<Scheme>) -> Result<(), ()> {
        self.write().scheme = scheme;
        Ok(())
    }

    fn authority(&self) -> Option<String> {
        self.read().authority.clone()
    }

    fn set_authority(&self, authority: Option<String>) -> Result<(), ()> {
        self.write().authority = authority;
        Ok(())
    }

    fn headers(&self) -> Headers {
        Headers::new(VirtFields {
            inner: RwLock::new(self.read().headers.get::<VirtFields>().read().clone()),
            is_mutable: false,
        })
    }
}

pub struct VirtRequestOptions {
    inner: RwLock<VirtRequestOptionsInner>,
}

#[expect(clippy::struct_field_names)]
struct VirtRequestOptionsInner {
    connect_timeout: Option<Duration>,
    first_byte_timeout: Option<Duration>,
    between_bytes_timeout: Option<Duration>,
}

impl VirtRequestOptions {
    #[expect(clippy::unwrap_used)]
    fn read(&self) -> RwLockReadGuard<'_, VirtRequestOptionsInner> {
        self.inner.read().unwrap()
    }

    #[expect(clippy::unwrap_used)]
    fn write(&self) -> RwLockWriteGuard<'_, VirtRequestOptionsInner> {
        self.inner.write().unwrap()
    }
}

impl GuestRequestOptions for VirtRequestOptions {
    fn new() -> Self {
        Self {
            inner: RwLock::new(VirtRequestOptionsInner {
                connect_timeout: None,
                first_byte_timeout: None,
                between_bytes_timeout: None,
            }),
        }
    }

    fn connect_timeout(&self) -> Option<Duration> {
        self.read().connect_timeout
    }

    fn set_connect_timeout(&self, duration: Option<Duration>) -> Result<(), ()> {
        self.write().connect_timeout = duration;
        Ok(())
    }

    fn first_byte_timeout(&self) -> Option<Duration> {
        self.read().first_byte_timeout
    }

    fn set_first_byte_timeout(&self, duration: Option<Duration>) -> Result<(), ()> {
        self.write().first_byte_timeout = duration;
        Ok(())
    }

    fn between_bytes_timeout(&self) -> Option<Duration> {
        self.read().between_bytes_timeout
    }

    fn set_between_bytes_timeout(&self, duration: Option<Duration>) -> Result<(), ()> {
        self.write().between_bytes_timeout = duration;
        Ok(())
    }
}

pub enum VirtResponseOutparam {}

impl GuestResponseOutparam for VirtResponseOutparam {
    fn set(_param: ResponseOutparam, _response: Result<OutgoingResponse, ErrorCode>) {
        // no-op
    }
}

pub enum VirtIncomingResponse {}

#[expect(clippy::uninhabited_references)] // FIXME
impl GuestIncomingResponse for VirtIncomingResponse {
    fn status(&self) -> StatusCode {
        match *self {}
    }

    fn headers(&self) -> Headers {
        match *self {}
    }

    fn consume(&self) -> Result<IncomingBody, ()> {
        match *self {}
    }
}

pub enum VirtIncomingBody {}

#[expect(clippy::uninhabited_references)] // FIXME
impl GuestIncomingBody for VirtIncomingBody {
    fn stream(&self) -> Result<InputStream, ()> {
        match *self {}
    }

    fn finish(this: IncomingBody) -> FutureTrailers {
        match *this.get::<Self>() {}
    }
}

pub enum VirtFutureTrailers {}

#[expect(clippy::uninhabited_references)] // FIXME
impl GuestFutureTrailers for VirtFutureTrailers {
    fn subscribe(&self) -> Pollable {
        match *self {}
    }

    fn get(&self) -> Option<Result<Result<Option<Trailers>, ErrorCode>, ()>> {
        match *self {}
    }
}

pub struct VirtOutgoingResponse {
    inner: RwLock<VirtOutgoingResponseInner>,
}

struct VirtOutgoingResponseInner {
    headers: Headers,
    status_code: StatusCode,
    has_body: bool,
}

impl VirtOutgoingResponse {
    #[expect(clippy::unwrap_used)]
    fn read(&self) -> RwLockReadGuard<'_, VirtOutgoingResponseInner> {
        self.inner.read().unwrap()
    }

    #[expect(clippy::unwrap_used)]
    fn write(&self) -> RwLockWriteGuard<'_, VirtOutgoingResponseInner> {
        self.inner.write().unwrap()
    }
}

impl GuestOutgoingResponse for VirtOutgoingResponse {
    fn new(headers: Headers) -> Self {
        Self {
            inner: RwLock::new(VirtOutgoingResponseInner {
                headers,
                status_code: 200,
                has_body: true,
            }),
        }
    }

    fn status_code(&self) -> StatusCode {
        self.read().status_code
    }

    fn set_status_code(&self, status_code: StatusCode) -> Result<(), ()> {
        self.write().status_code = status_code;
        Ok(())
    }

    fn headers(&self) -> Headers {
        Headers::new(VirtFields {
            inner: RwLock::new(self.read().headers.get::<VirtFields>().read().clone()),
            is_mutable: false,
        })
    }

    fn body(&self) -> Result<OutgoingBody, ()> {
        let mut inner = self.write();

        if !inner.has_body {
            return Err(());
        }

        inner.has_body = false;
        std::mem::drop(inner);

        Ok(OutgoingBody::new(VirtOutgoingBody {
            inner: Mutex::new(VirtOutgoingBodyInner { has_written: false }),
        }))
    }
}

pub struct VirtOutgoingBody {
    inner: Mutex<VirtOutgoingBodyInner>,
}

struct VirtOutgoingBodyInner {
    has_written: bool,
}

impl VirtOutgoingBody {
    #[expect(clippy::unwrap_used)]
    fn write(&self) -> MutexGuard<'_, VirtOutgoingBodyInner> {
        self.inner.lock().unwrap()
    }
}

impl GuestOutgoingBody for VirtOutgoingBody {
    fn write(&self) -> Result<OutputStream, ()> {
        let mut inner = self.write();

        if inner.has_written {
            return Err(());
        }

        inner.has_written = true;
        std::mem::drop(inner);

        Ok(output_sink())
    }

    fn finish(_this: OutgoingBody, _trailers: Option<Trailers>) -> Result<(), ErrorCode> {
        Ok(())
    }
}

pub enum VirtFutureIncomingResponse {}

#[expect(clippy::uninhabited_references)] // FIXME
impl GuestFutureIncomingResponse for VirtFutureIncomingResponse {
    fn subscribe(&self) -> Pollable {
        match *self {}
    }

    fn get(&self) -> Option<Result<Result<IncomingResponse, ErrorCode>, ()>> {
        match *self {}
    }
}
