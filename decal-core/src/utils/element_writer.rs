use super::writer::Writer;
use crate::{
    primitives::{
        Length,
        PositiveF32Pair,
    },
    utils::FloatWriter,
};
use std::{
    fmt::{
        Display,
        Write,
    },
    marker::PhantomData,
};
use strict_num::{
    FiniteF32,
    NormalizedF32,
    PositiveF32,
};

pub(crate) struct Initialized;
pub(crate) struct Opened;

mod private {
    pub trait WriterState {}
}

impl private::WriterState for Initialized {}
impl private::WriterState for Opened {}

#[derive(Debug)]
pub(crate) struct ElementWriter<'a, T, S>
where
    T: Write,
{
    out: &'a mut T,
    element_name: &'a str,
    state: PhantomData<S>,
}

impl<T, S> FloatWriter<T> for ElementWriter<'_, T, S>
where
    T: Write,
{
    fn out_mut(&mut self) -> &mut T {
        &mut self.out
    }
}

impl<T, S> Writer<T> for ElementWriter<'_, T, S> where T: Write {}

impl<'a, T, S> ElementWriter<'a, T, S>
where
    T: Write,
    S: private::WriterState,
{
    pub(crate) fn write<F>(self, write_fn: F) -> Result<Self, std::fmt::Error>
    where
        F: FnOnce(&mut T) -> std::fmt::Result,
    {
        write_fn(self.out).map(|_| self)
    }
}

impl<'a, T> ElementWriter<'a, T, Initialized>
where
    T: Write,
{
    pub(crate) fn new(
        out: &'a mut T,
        element_name: &'a str,
    ) -> Result<ElementWriter<'a, T, Initialized>, std::fmt::Error> {
        write!(out, "<{element_name}").map(|_| ElementWriter {
            out,
            element_name,
            state: PhantomData,
        })
    }

    pub(crate) fn attr<V>(self, key: &str, value: V) -> Result<Self, std::fmt::Error>
    where
        V: IntoAttrValue<T>,
    {
        value.write(key, self.out)?;
        Ok(self)
    }

    pub(crate) fn attrs<'b, I, V>(self, attrs: I) -> Result<Self, std::fmt::Error>
    where
        V: IntoAttrValue<T>,
        I: IntoIterator<Item = (&'b str, V)>,
    {
        attrs
            .into_iter()
            .try_for_each(|(key, value)| value.write(key, self.out))
            .map(|_| self)
    }

    pub(crate) fn attr_if<V>(
        mut self,
        key: &str,
        value: V,
        condition: bool,
    ) -> Result<Self, std::fmt::Error>
    where
        V: IntoAttrValue<T>,
    {
        if condition {
            self = self.attr(key, value)?;
        }

        Ok(self)
    }

    pub(crate) fn write_attr<F>(self, key: &str, write_fn: F) -> Result<Self, std::fmt::Error>
    where
        F: FnOnce(&mut T) -> std::fmt::Result,
    {
        write!(self.out, r#" {key}=""#)?;
        write_fn(self.out)?;
        self.out.write_char('"').map(|_| self)
    }

    pub(crate) fn open(self) -> Result<ElementWriter<'a, T, Opened>, std::fmt::Error> {
        self.out.write_char('>').map(|_| ElementWriter {
            out: self.out,
            element_name: self.element_name,
            state: PhantomData,
        })
    }

    pub(crate) fn content<F>(
        self,
        write_fn: F,
    ) -> Result<ElementWriter<'a, T, Opened>, std::fmt::Error>
    where
        F: FnOnce(&mut T) -> std::fmt::Result,
    {
        let writer = self.open()?;
        writer.content(write_fn)
    }

    pub(crate) fn close(self) -> std::fmt::Result {
        self.out.write_str(" />")
    }

    pub(crate) fn close_tag(out: &'a mut T, element_name: &str) -> std::fmt::Result {
        write!(out, "</{element_name}>")
    }
}

impl<'a, T> ElementWriter<'a, T, Opened>
where
    T: Write,
{
    pub(crate) fn content<F>(self, write_fn: F) -> Result<Self, std::fmt::Error>
    where
        F: FnOnce(&mut T) -> std::fmt::Result,
    {
        self.write(write_fn)
    }

    pub(crate) fn close(self) -> std::fmt::Result {
        write!(self.out, "</{}>", self.element_name)
    }
}

//

pub(crate) trait IntoAttrValue<W>
where
    W: Write,
{
    fn write(&self, key: &str, out: &mut W) -> std::fmt::Result;
}

impl<'a, W, D> IntoAttrValue<W> for (D,)
where
    W: Write,
    D: Display,
{
    fn write(&self, key: &str, out: &mut W) -> std::fmt::Result {
        write!(out, r#" {key}="{}""#, self.0)
    }
}

impl<'a, W> IntoAttrValue<W> for &'a str
where
    W: Write,
{
    fn write(&self, key: &str, out: &mut W) -> std::fmt::Result {
        write!(out, r#" {key}="{self}""#)
    }
}

impl<'a, W> IntoAttrValue<W> for String
where
    W: Write,
{
    fn write(&self, key: &str, out: &mut W) -> std::fmt::Result {
        IntoAttrValue::write(&self.as_str(), key, out)
    }
}

impl<'a, W> IntoAttrValue<W> for f32
where
    W: Write,
{
    fn write(&self, key: &str, out: &mut W) -> std::fmt::Result {
        write!(out, r#" {key}=""#)?;
        out.write_float(*self)?;
        out.write_char('"')
    }
}

impl<'a, W, const AUTO: bool, const PERCENT: bool> IntoAttrValue<W> for Length<AUTO, PERCENT>
where
    W: Write,
{
    fn write(&self, key: &str, out: &mut W) -> std::fmt::Result {
        IntoAttrValue::write(&(self,), key, out)
    }
}

impl<'a, W> IntoAttrValue<W> for FiniteF32
where
    W: Write,
{
    fn write(&self, key: &str, out: &mut W) -> std::fmt::Result {
        IntoAttrValue::write(&self.get(), key, out)
    }
}

impl<'a, W> IntoAttrValue<W> for NormalizedF32
where
    W: Write,
{
    fn write(&self, key: &str, out: &mut W) -> std::fmt::Result {
        IntoAttrValue::write(&self.get(), key, out)
    }
}

impl<'a, W> IntoAttrValue<W> for PositiveF32
where
    W: Write,
{
    fn write(&self, key: &str, out: &mut W) -> std::fmt::Result {
        IntoAttrValue::write(&self.get(), key, out)
    }
}

impl<'a, W> IntoAttrValue<W> for PositiveF32Pair
where
    W: Write,
{
    fn write(&self, key: &str, out: &mut W) -> std::fmt::Result {
        write!(out, r#" {key}="{self}""#)
    }
}

impl<'a, W, V> IntoAttrValue<W> for Option<V>
where
    W: Write,
    V: IntoAttrValue<W>,
{
    fn write(&self, key: &str, out: &mut W) -> std::fmt::Result {
        if let Some(value) = self {
            IntoAttrValue::write(value, key, out)?;
        }

        Ok(())
    }
}
