package routing

import (
	"net/http"

	"github.com/arielkeren/lyra/middleware"
)

func Route(pattern string, handler func(http.ResponseWriter, *http.Request)) {
	http.HandleFunc(pattern, middleware.CorsMiddleware(handler))
}
