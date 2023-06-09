// Code generated by protoc-gen-validate. DO NOT EDIT.
// source: proto/message.proto

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

// Validate checks the field values on TestRequest with the rules defined in
// the proto definition for this message. If any rules are violated, the first
// error encountered is returned, or nil if there are no violations.
func (m *TestRequest) Validate() error {
	return m.validate(false)
}

// ValidateAll checks the field values on TestRequest with the rules defined in
// the proto definition for this message. If any rules are violated, the
// result is a list of violation errors wrapped in TestRequestMultiError, or
// nil if none found.
func (m *TestRequest) ValidateAll() error {
	return m.validate(true)
}

func (m *TestRequest) validate(all bool) error {
	if m == nil {
		return nil
	}

	var errors []error

	if l := utf8.RuneCountInString(m.GetName()); l < 1 || l > 33 {
		err := TestRequestValidationError{
			field:  "Name",
			reason: "value length must be between 1 and 33 runes, inclusive",
		}
		if !all {
			return err
		}
		errors = append(errors, err)
	}

	// no validation rules for Age

	if all {
		switch v := interface{}(m.GetCommonMessage()).(type) {
		case interface{ ValidateAll() error }:
			if err := v.ValidateAll(); err != nil {
				errors = append(errors, TestRequestValidationError{
					field:  "CommonMessage",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		case interface{ Validate() error }:
			if err := v.Validate(); err != nil {
				errors = append(errors, TestRequestValidationError{
					field:  "CommonMessage",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		}
	} else if v, ok := interface{}(m.GetCommonMessage()).(interface{ Validate() error }); ok {
		if err := v.Validate(); err != nil {
			return TestRequestValidationError{
				field:  "CommonMessage",
				reason: "embedded message failed validation",
				cause:  err,
			}
		}
	}

	if all {
		switch v := interface{}(m.GetStartTime()).(type) {
		case interface{ ValidateAll() error }:
			if err := v.ValidateAll(); err != nil {
				errors = append(errors, TestRequestValidationError{
					field:  "StartTime",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		case interface{ Validate() error }:
			if err := v.Validate(); err != nil {
				errors = append(errors, TestRequestValidationError{
					field:  "StartTime",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		}
	} else if v, ok := interface{}(m.GetStartTime()).(interface{ Validate() error }); ok {
		if err := v.Validate(); err != nil {
			return TestRequestValidationError{
				field:  "StartTime",
				reason: "embedded message failed validation",
				cause:  err,
			}
		}
	}

	if all {
		switch v := interface{}(m.GetSubMchId()).(type) {
		case interface{ ValidateAll() error }:
			if err := v.ValidateAll(); err != nil {
				errors = append(errors, TestRequestValidationError{
					field:  "SubMchId",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		case interface{ Validate() error }:
			if err := v.Validate(); err != nil {
				errors = append(errors, TestRequestValidationError{
					field:  "SubMchId",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		}
	} else if v, ok := interface{}(m.GetSubMchId()).(interface{ Validate() error }); ok {
		if err := v.Validate(); err != nil {
			return TestRequestValidationError{
				field:  "SubMchId",
				reason: "embedded message failed validation",
				cause:  err,
			}
		}
	}

	{
		sorted_keys := make([]string, len(m.GetAttrs()))
		i := 0
		for key := range m.GetAttrs() {
			sorted_keys[i] = key
			i++
		}
		sort.Slice(sorted_keys, func(i, j int) bool { return sorted_keys[i] < sorted_keys[j] })
		for _, key := range sorted_keys {
			val := m.GetAttrs()[key]
			_ = val

			// no validation rules for Attrs[key]

			if all {
				switch v := interface{}(val).(type) {
				case interface{ ValidateAll() error }:
					if err := v.ValidateAll(); err != nil {
						errors = append(errors, TestRequestValidationError{
							field:  fmt.Sprintf("Attrs[%v]", key),
							reason: "embedded message failed validation",
							cause:  err,
						})
					}
				case interface{ Validate() error }:
					if err := v.Validate(); err != nil {
						errors = append(errors, TestRequestValidationError{
							field:  fmt.Sprintf("Attrs[%v]", key),
							reason: "embedded message failed validation",
							cause:  err,
						})
					}
				}
			} else if v, ok := interface{}(val).(interface{ Validate() error }); ok {
				if err := v.Validate(); err != nil {
					return TestRequestValidationError{
						field:  fmt.Sprintf("Attrs[%v]", key),
						reason: "embedded message failed validation",
						cause:  err,
					}
				}
			}

		}
	}

	// no validation rules for ResultCode

	// no validation rules for Image

	// no validation rules for Test

	if all {
		switch v := interface{}(m.GetTargetTime()).(type) {
		case interface{ ValidateAll() error }:
			if err := v.ValidateAll(); err != nil {
				errors = append(errors, TestRequestValidationError{
					field:  "TargetTime",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		case interface{ Validate() error }:
			if err := v.Validate(); err != nil {
				errors = append(errors, TestRequestValidationError{
					field:  "TargetTime",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		}
	} else if v, ok := interface{}(m.GetTargetTime()).(interface{ Validate() error }); ok {
		if err := v.Validate(); err != nil {
			return TestRequestValidationError{
				field:  "TargetTime",
				reason: "embedded message failed validation",
				cause:  err,
			}
		}
	}

	// no validation rules for Test6

	switch v := m.TestOneof.(type) {
	case *TestRequest_Gaga:
		if v == nil {
			err := TestRequestValidationError{
				field:  "TestOneof",
				reason: "oneof value cannot be a typed-nil",
			}
			if !all {
				return err
			}
			errors = append(errors, err)
		}
		// no validation rules for Gaga
	case *TestRequest_Haha:
		if v == nil {
			err := TestRequestValidationError{
				field:  "TestOneof",
				reason: "oneof value cannot be a typed-nil",
			}
			if !all {
				return err
			}
			errors = append(errors, err)
		}
		// no validation rules for Haha
	default:
		_ = v // ensures v is used
	}

	if len(errors) > 0 {
		return TestRequestMultiError(errors)
	}

	return nil
}

// TestRequestMultiError is an error wrapping multiple validation errors
// returned by TestRequest.ValidateAll() if the designated constraints aren't met.
type TestRequestMultiError []error

// Error returns a concatenation of all the error messages it wraps.
func (m TestRequestMultiError) Error() string {
	var msgs []string
	for _, err := range m {
		msgs = append(msgs, err.Error())
	}
	return strings.Join(msgs, "; ")
}

// AllErrors returns a list of validation violation errors.
func (m TestRequestMultiError) AllErrors() []error { return m }

// TestRequestValidationError is the validation error returned by
// TestRequest.Validate if the designated constraints aren't met.
type TestRequestValidationError struct {
	field  string
	reason string
	cause  error
	key    bool
}

// Field function returns field value.
func (e TestRequestValidationError) Field() string { return e.field }

// Reason function returns reason value.
func (e TestRequestValidationError) Reason() string { return e.reason }

// Cause function returns cause value.
func (e TestRequestValidationError) Cause() error { return e.cause }

// Key function returns key value.
func (e TestRequestValidationError) Key() bool { return e.key }

// ErrorName returns error name.
func (e TestRequestValidationError) ErrorName() string { return "TestRequestValidationError" }

// Error satisfies the builtin error interface
func (e TestRequestValidationError) Error() string {
	cause := ""
	if e.cause != nil {
		cause = fmt.Sprintf(" | caused by: %v", e.cause)
	}

	key := ""
	if e.key {
		key = "key for "
	}

	return fmt.Sprintf(
		"invalid %sTestRequest.%s: %s%s",
		key,
		e.field,
		e.reason,
		cause)
}

var _ error = TestRequestValidationError{}

var _ interface {
	Field() string
	Reason() string
	Key() bool
	Cause() error
	ErrorName() string
} = TestRequestValidationError{}

// Validate checks the field values on TestResponse with the rules defined in
// the proto definition for this message. If any rules are violated, the first
// error encountered is returned, or nil if there are no violations.
func (m *TestResponse) Validate() error {
	return m.validate(false)
}

// ValidateAll checks the field values on TestResponse with the rules defined
// in the proto definition for this message. If any rules are violated, the
// result is a list of violation errors wrapped in TestResponseMultiError, or
// nil if none found.
func (m *TestResponse) ValidateAll() error {
	return m.validate(true)
}

func (m *TestResponse) validate(all bool) error {
	if m == nil {
		return nil
	}

	var errors []error

	if l := utf8.RuneCountInString(m.GetName()); l < 1 || l > 33 {
		err := TestResponseValidationError{
			field:  "Name",
			reason: "value length must be between 1 and 33 runes, inclusive",
		}
		if !all {
			return err
		}
		errors = append(errors, err)
	}

	// no validation rules for Age

	if all {
		switch v := interface{}(m.GetCommonMessage()).(type) {
		case interface{ ValidateAll() error }:
			if err := v.ValidateAll(); err != nil {
				errors = append(errors, TestResponseValidationError{
					field:  "CommonMessage",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		case interface{ Validate() error }:
			if err := v.Validate(); err != nil {
				errors = append(errors, TestResponseValidationError{
					field:  "CommonMessage",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		}
	} else if v, ok := interface{}(m.GetCommonMessage()).(interface{ Validate() error }); ok {
		if err := v.Validate(); err != nil {
			return TestResponseValidationError{
				field:  "CommonMessage",
				reason: "embedded message failed validation",
				cause:  err,
			}
		}
	}

	if all {
		switch v := interface{}(m.GetStartTime()).(type) {
		case interface{ ValidateAll() error }:
			if err := v.ValidateAll(); err != nil {
				errors = append(errors, TestResponseValidationError{
					field:  "StartTime",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		case interface{ Validate() error }:
			if err := v.Validate(); err != nil {
				errors = append(errors, TestResponseValidationError{
					field:  "StartTime",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		}
	} else if v, ok := interface{}(m.GetStartTime()).(interface{ Validate() error }); ok {
		if err := v.Validate(); err != nil {
			return TestResponseValidationError{
				field:  "StartTime",
				reason: "embedded message failed validation",
				cause:  err,
			}
		}
	}

	if all {
		switch v := interface{}(m.GetSubMchId()).(type) {
		case interface{ ValidateAll() error }:
			if err := v.ValidateAll(); err != nil {
				errors = append(errors, TestResponseValidationError{
					field:  "SubMchId",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		case interface{ Validate() error }:
			if err := v.Validate(); err != nil {
				errors = append(errors, TestResponseValidationError{
					field:  "SubMchId",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		}
	} else if v, ok := interface{}(m.GetSubMchId()).(interface{ Validate() error }); ok {
		if err := v.Validate(); err != nil {
			return TestResponseValidationError{
				field:  "SubMchId",
				reason: "embedded message failed validation",
				cause:  err,
			}
		}
	}

	{
		sorted_keys := make([]string, len(m.GetAttrs()))
		i := 0
		for key := range m.GetAttrs() {
			sorted_keys[i] = key
			i++
		}
		sort.Slice(sorted_keys, func(i, j int) bool { return sorted_keys[i] < sorted_keys[j] })
		for _, key := range sorted_keys {
			val := m.GetAttrs()[key]
			_ = val

			// no validation rules for Attrs[key]

			if all {
				switch v := interface{}(val).(type) {
				case interface{ ValidateAll() error }:
					if err := v.ValidateAll(); err != nil {
						errors = append(errors, TestResponseValidationError{
							field:  fmt.Sprintf("Attrs[%v]", key),
							reason: "embedded message failed validation",
							cause:  err,
						})
					}
				case interface{ Validate() error }:
					if err := v.Validate(); err != nil {
						errors = append(errors, TestResponseValidationError{
							field:  fmt.Sprintf("Attrs[%v]", key),
							reason: "embedded message failed validation",
							cause:  err,
						})
					}
				}
			} else if v, ok := interface{}(val).(interface{ Validate() error }); ok {
				if err := v.Validate(); err != nil {
					return TestResponseValidationError{
						field:  fmt.Sprintf("Attrs[%v]", key),
						reason: "embedded message failed validation",
						cause:  err,
					}
				}
			}

		}
	}

	// no validation rules for ResultCode

	// no validation rules for Image

	switch v := m.TestOneof.(type) {
	case *TestResponse_Gaga:
		if v == nil {
			err := TestResponseValidationError{
				field:  "TestOneof",
				reason: "oneof value cannot be a typed-nil",
			}
			if !all {
				return err
			}
			errors = append(errors, err)
		}
		// no validation rules for Gaga
	case *TestResponse_Haha:
		if v == nil {
			err := TestResponseValidationError{
				field:  "TestOneof",
				reason: "oneof value cannot be a typed-nil",
			}
			if !all {
				return err
			}
			errors = append(errors, err)
		}
		// no validation rules for Haha
	default:
		_ = v // ensures v is used
	}

	if len(errors) > 0 {
		return TestResponseMultiError(errors)
	}

	return nil
}

// TestResponseMultiError is an error wrapping multiple validation errors
// returned by TestResponse.ValidateAll() if the designated constraints aren't met.
type TestResponseMultiError []error

// Error returns a concatenation of all the error messages it wraps.
func (m TestResponseMultiError) Error() string {
	var msgs []string
	for _, err := range m {
		msgs = append(msgs, err.Error())
	}
	return strings.Join(msgs, "; ")
}

// AllErrors returns a list of validation violation errors.
func (m TestResponseMultiError) AllErrors() []error { return m }

// TestResponseValidationError is the validation error returned by
// TestResponse.Validate if the designated constraints aren't met.
type TestResponseValidationError struct {
	field  string
	reason string
	cause  error
	key    bool
}

// Field function returns field value.
func (e TestResponseValidationError) Field() string { return e.field }

// Reason function returns reason value.
func (e TestResponseValidationError) Reason() string { return e.reason }

// Cause function returns cause value.
func (e TestResponseValidationError) Cause() error { return e.cause }

// Key function returns key value.
func (e TestResponseValidationError) Key() bool { return e.key }

// ErrorName returns error name.
func (e TestResponseValidationError) ErrorName() string { return "TestResponseValidationError" }

// Error satisfies the builtin error interface
func (e TestResponseValidationError) Error() string {
	cause := ""
	if e.cause != nil {
		cause = fmt.Sprintf(" | caused by: %v", e.cause)
	}

	key := ""
	if e.key {
		key = "key for "
	}

	return fmt.Sprintf(
		"invalid %sTestResponse.%s: %s%s",
		key,
		e.field,
		e.reason,
		cause)
}

var _ error = TestResponseValidationError{}

var _ interface {
	Field() string
	Reason() string
	Key() bool
	Cause() error
	ErrorName() string
} = TestResponseValidationError{}
