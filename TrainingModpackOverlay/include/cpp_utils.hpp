#pragma once

#include <type_traits>
#include <cstdint>
#include <array>

namespace detail
{
template<class T, typename U = uint32_t> struct BitFlags
{
	using UnderlyingType = U;
	using Type           = T;

	constexpr static const UnderlyingType s_validMask = (UnderlyingType{1u << *Type::Count} - UnderlyingType{1});

	struct NoneType
	{};
	struct AllType
	{};

	UnderlyingType m_bits;

	constexpr static const AllType  All{};
	constexpr static const NoneType None{};

	BitFlags()                = default;
	BitFlags(const BitFlags&) = default;

	constexpr BitFlags(NoneType) : m_bits(0) {}
	constexpr BitFlags(AllType) : m_bits(s_validMask) {}
	constexpr explicit BitFlags(UnderlyingType t) : m_bits(s_validMask & t) {}

	BitFlags& operator=(const BitFlags&) = default;
	~BitFlags()                          = default;

	constexpr BitFlags(Type t) : m_bits{UnderlyingType{1} << static_cast<std::underlying_type_t<Type>>(t)} {}

	constexpr BitFlags& operator|=(BitFlags rhs)
	{
		m_bits |= rhs.m_bits;
		return *this;
	}
	constexpr BitFlags& operator&=(BitFlags rhs)
	{
		m_bits &= rhs.m_bits;
		return *this;
	}
	constexpr BitFlags& operator-=(BitFlags rhs)
	{
		m_bits &= ~rhs.m_bits;
		return *this;
	}

	explicit operator bool() const { return m_bits != 0; }
};

template<typename T, typename U> constexpr BitFlags<T, U> operator|(BitFlags<T, U> lhs, BitFlags<T, U> rhs)
{
	lhs |= rhs;
	return lhs;
}

template<typename T, typename U> constexpr BitFlags<T, U> operator&(BitFlags<T, U> lhs, BitFlags<T, U> rhs)
{
	lhs &= rhs;
	return lhs;
}

template<typename T, typename U> constexpr BitFlags<T, U> operator-(BitFlags<T, U> lhs, BitFlags<T, U> rhs)
{
	lhs -= rhs;
	return lhs;
}
template<typename T, typename U> constexpr BitFlags<T, U> operator-(BitFlags<T, U> lhs, T rhs)
{
	lhs -= rhs;
	return lhs;
}

template<typename T, typename U> constexpr BitFlags<T, U> operator-(T lhs, BitFlags<T, U> rhs)
{
	rhs -= lhs;
	return rhs;
}

template<typename T, typename U> constexpr BitFlags<T, U> operator|(BitFlags<T, U> lhs, T rhs)
{
	lhs |= rhs;
	return lhs;
}

template<typename T, typename U> constexpr BitFlags<T, U> operator|(T lhs, BitFlags<T, U> rhs)
{
	rhs |= lhs;
	return rhs;
}

template<typename T, typename U> constexpr bool operator&(BitFlags<T, U> lhs, T rhs)
{
	lhs &= rhs;
	return lhs.m_bits != 0;
}

template<typename T, typename U> constexpr bool operator&(T lhs, BitFlags<T, U> rhs)
{
	rhs &= lhs;
	return rhs.m_bits != 0;
}

template<typename T> struct FlagType;

template<typename T> using FlagTypeT = typename FlagType<T>::Type;

template<class T> struct EnumArray;

} // namespace detail

#define DEFINE_FLAGS_UT(TypeName, UnderlyingType)                                                               \
	using TypeName##s = detail::BitFlags<TypeName, UnderlyingType>;                                             \
	constexpr TypeName##s operator|(TypeName lhs, TypeName rhs) { return TypeName##s{lhs} | TypeName##s{rhs}; } \
	namespace detail                                                                                            \
	{                                                                                                           \
	template<> struct FlagType<TypeName>                                                                        \
	{                                                                                                           \
		using Type = TypeName##s;                                                                               \
	};                                                                                                          \
	}

#define DEFINE_FLAGS(TypeName) DEFINE_FLAGS_UT(TypeName, uint32_t)
#define ENUM_X(type, e, y) e,
#define ENUM_QUAL_X(type, e, y) type::e,
#define ENUM_NAME_PAIR(type, e, y) {type::e, y},
#define CASE_X(type, e, y) \
	case type::e:          \
		return y;
#define DEFINE_ENUM_CLASS_TYPE(Name, Type)                                                    \
	enum class Name : Type                                                                    \
	{                                                                                         \
		ENUM_CLASS_##Name(Name, ENUM_X) Count                                                 \
	};                                                                                        \
	constexpr auto operator*(Name t) { return static_cast<std::underlying_type_t<Name>>(t); } \
	namespace detail                                                                          \
	{                                                                                         \
	template<> struct EnumArray<Name>                                                         \
	{                                                                                         \
		constexpr static const std::array<std::pair<Name, const char*>, *Name::Count> values{ \
		    {ENUM_CLASS_##Name(Name, ENUM_NAME_PAIR)}};                                       \
	};                                                                                        \
	}                                                                                         \
	constexpr const char* toString(Name t)                                                    \
	{                                                                                         \
		if(t < Name::Count)                                                                   \
		{                                                                                     \
			return detail::EnumArray<Name>::values[*t].second;                                \
		}                                                                                     \
		return "UNKNWON";                                                                     \
	}                                                                                         \
	DEFINE_FLAGS_UT(Name, Type)
#define DEFINE_ENUM_CLASS(Name) DEFINE_ENUM_CLASS_TYPE(Name, uint32_t)
