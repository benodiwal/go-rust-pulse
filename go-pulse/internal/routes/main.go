package routes

import "github.com/gin-gonic/gin"

type Router struct {
	Engine *gin.Engine
}

func New() *Router {
	return &Router{
		Engine: gin.New(),
	}
}
