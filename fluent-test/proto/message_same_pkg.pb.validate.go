// Code generated by protoc-gen-validate. DO NOT EDIT.
// source: proto/message_same_pkg.proto

package demopb

import (
	"bytes"
	"errors"
	"fmt"
	"net"
	"net/mail"
	"net/url"
	"regexp"
	"sort"
	"strings"
	"time"
	"unicode/utf8"

	"google.golang.org/protobuf/types/known/anypb"
)

// ensure the imports are used
var (
	_ = bytes.MinRead
	_ = errors.New("")
	_ = fmt.Print
	_ = utf8.UTFMax
	_ = (*regexp.Regexp)(nil)
	_ = (*strings.Reader)(nil)
	_ = net.IPv4len
	_ = time.Duration(0)
	_ = (*url.URL)(nil)
	_ = (*mail.Address)(nil)
	_ = anypb.Any{}
	_ = sort.Sort
)

// Validate checks the field values on TestSamePkgRequest with the rules
// defined in the proto definition for this message. If any rules are
// violated, the first error encountered is returned, or nil if there are no violations.
func (m *TestSamePkgRequest) Validate() error {
	return m.validate(false)
}

// ValidateAll checks the field values on TestSamePkgRequest with the rules
// defined in the proto definition for this message. If any rules are
// violated, the result is a list of violation errors wrapped in
// TestSamePkgRequestMultiError, or nil if none found.
func (m *TestSamePkgRequest) ValidateAll() error {
	return m.validate(true)
}

func (m *TestSamePkgRequest) validate(all bool) error {
	if m == nil {
		return nil
	}

	var errors []error

	// no validation rules for Name

	// no validation rules for Age

	if all {
		switch v := interface{}(m.GetCommonMessage()).(type) {
		case interface{ ValidateAll() error }:
			if err := v.ValidateAll(); err != nil {
				errors = append(errors, TestSamePkgRequestValidationError{
					field:  "CommonMessage",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		case interface{ Validate() error }:
			if err := v.Validate(); err != nil {
				errors = append(errors, TestSamePkgRequestValidationError{
					field:  "CommonMessage",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		}
	} else if v, ok := interface{}(m.GetCommonMessage()).(interface{ Validate() error }); ok {
		if err := v.Validate(); err != nil {
			return TestSamePkgRequestValidationError{
				field:  "CommonMessage",
				reason: "embedded message failed validation",
				cause:  err,
			}
		}
	}

	if len(errors) > 0 {
		return TestSamePkgRequestMultiError(errors)
	}

	return nil
}

// TestSamePkgRequestMultiError is an error wrapping multiple validation errors
// returned by TestSamePkgRequest.ValidateAll() if the designated constraints
// aren't met.
type TestSamePkgRequestMultiError []error

// Error returns a concatenation of all the error messages it wraps.
func (m TestSamePkgRequestMultiError) Error() string {
	var msgs []string
	for _, err := range m {
		msgs = append(msgs, err.Error())
	}
	return strings.Join(msgs, "; ")
}

// AllErrors returns a list of validation violation errors.
func (m TestSamePkgRequestMultiError) AllErrors() []error { return m }

// TestSamePkgRequestValidationError is the validation error returned by
// TestSamePkgRequest.Validate if the designated constraints aren't met.
type TestSamePkgRequestValidationError struct {
	field  string
	reason string
	cause  error
	key    bool
}

// Field function returns field value.
func (e TestSamePkgRequestValidationError) Field() string { return e.field }

// Reason function returns reason value.
func (e TestSamePkgRequestValidationError) Reason() string { return e.reason }

// Cause function returns cause value.
func (e TestSamePkgRequestValidationError) Cause() error { return e.cause }

// Key function returns key value.
func (e TestSamePkgRequestValidationError) Key() bool { return e.key }

// ErrorName returns error name.
func (e TestSamePkgRequestValidationError) ErrorName() string {
	return "TestSamePkgRequestValidationError"
}

// Error satisfies the builtin error interface
func (e TestSamePkgRequestValidationError) Error() string {
	cause := ""
	if e.cause != nil {
		cause = fmt.Sprintf(" | caused by: %v", e.cause)
	}

	key := ""
	if e.key {
		key = "key for "
	}

	return fmt.Sprintf(
		"invalid %sTestSamePkgRequest.%s: %s%s",
		key,
		e.field,
		e.reason,
		cause)
}

var _ error = TestSamePkgRequestValidationError{}

var _ interface {
	Field() string
	Reason() string
	Key() bool
	Cause() error
	ErrorName() string
} = TestSamePkgRequestValidationError{}

// Validate checks the field values on TestSamePkgResponse with the rules
// defined in the proto definition for this message. If any rules are
// violated, the first error encountered is returned, or nil if there are no violations.
func (m *TestSamePkgResponse) Validate() error {
	return m.validate(false)
}

// ValidateAll checks the field values on TestSamePkgResponse with the rules
// defined in the proto definition for this message. If any rules are
// violated, the result is a list of violation errors wrapped in
// TestSamePkgResponseMultiError, or nil if none found.
func (m *TestSamePkgResponse) ValidateAll() error {
	return m.validate(true)
}

func (m *TestSamePkgResponse) validate(all bool) error {
	if m == nil {
		return nil
	}

	var errors []error

	// no validation rules for ResultCode

	// no validation rules for Name

	// no validation rules for Age

	if len(errors) > 0 {
		return TestSamePkgResponseMultiError(errors)
	}

	return nil
}

// TestSamePkgResponseMultiError is an error wrapping multiple validation
// errors returned by TestSamePkgResponse.ValidateAll() if the designated
// constraints aren't met.
type TestSamePkgResponseMultiError []error

// Error returns a concatenation of all the error messages it wraps.
func (m TestSamePkgResponseMultiError) Error() string {
	var msgs []string
	for _, err := range m {
		msgs = append(msgs, err.Error())
	}
	return strings.Join(msgs, "; ")
}

// AllErrors returns a list of validation violation errors.
func (m TestSamePkgResponseMultiError) AllErrors() []error { return m }

// TestSamePkgResponseValidationError is the validation error returned by
// TestSamePkgResponse.Validate if the designated constraints aren't met.
type TestSamePkgResponseValidationError struct {
	field  string
	reason string
	cause  error
	key    bool
}

// Field function returns field value.
func (e TestSamePkgResponseValidationError) Field() string { return e.field }

// Reason function returns reason value.
func (e TestSamePkgResponseValidationError) Reason() string { return e.reason }

// Cause function returns cause value.
func (e TestSamePkgResponseValidationError) Cause() error { return e.cause }

// Key function returns key value.
func (e TestSamePkgResponseValidationError) Key() bool { return e.key }

// ErrorName returns error name.
func (e TestSamePkgResponseValidationError) ErrorName() string {
	return "TestSamePkgResponseValidationError"
}

// Error satisfies the builtin error interface
func (e TestSamePkgResponseValidationError) Error() string {
	cause := ""
	if e.cause != nil {
		cause = fmt.Sprintf(" | caused by: %v", e.cause)
	}

	key := ""
	if e.key {
		key = "key for "
	}

	return fmt.Sprintf(
		"invalid %sTestSamePkgResponse.%s: %s%s",
		key,
		e.field,
		e.reason,
		cause)
}

var _ error = TestSamePkgResponseValidationError{}

var _ interface {
	Field() string
	Reason() string
	Key() bool
	Cause() error
	ErrorName() string
} = TestSamePkgResponseValidationError{}
