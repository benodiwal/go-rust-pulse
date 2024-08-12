package routes

import (
	"net/http"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
)

func (r *Router) RegisterMiddlewares() {
	r.Engine.Use(cors.Default())
}

func (r *Router) RegisterRoutes() {
	r.Engine.GET("/", func(ctx *gin.Context) {
		ctx.String(http.StatusOK, "OK")
	})

	r.Engine.GET("/health", func(ctx *gin.Context) {
		ctx.String(http.StatusOK, "OK")
	})
}
