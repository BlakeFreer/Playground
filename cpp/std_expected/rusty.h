/// @author Blake Freer
/// @date 2024-10-23

#include <expected>
#include <functional>
#include <optional>

namespace rusty {

/* std::expected -> Result */
template <typename T, typename E>
using Result = std::expected<T, E>;

template <typename E>
using Err = std::unexpected<E>;

// C++ does not have a "bottom type" equivalent to ! in Rust. This means that
// we cannot easily "panic!" in one of the handler functions, since c++ still
// expects the Handler to return a valid type.

template <typename T, typename E, typename R>
R match(Result<T, E> result, std::function<R(T)> OkHandler,
        std::function<R(E)> ErrHandler) {
    if (result.has_value()) {
        return OkHandler(result.value());
    } else {
        // This is not exactly how Rust matches Results as it would pattern
        // match the error here rather than passing to another function.
        return ErrHandler(result.error());
    }
}

/* std::optional -> Option */
template <typename T>
using Option = std::optional<T>;
constexpr auto None = std::nullopt;

template <typename T, typename R>
R match(Option<T> opt, std::function<R(T)> SomeHandler,
        std::function<R()> NoneHandler) {
    if (opt.has_value()) {
        return SomeHandler(opt.value());
    } else {
        return NoneHandler();
    }
}

}  // namespace rusty