package app

import (
	"fmt"

	"github.com/benodiwal/go-pulse/internal/env"
	"github.com/benodiwal/go-pulse/internal/routes"
)

func Run() {
	env.Load()

	router := routes.New()
	router.RegisterMiddlewares()
	router.RegisterRoutes()

	router.Engine.Run(fmt.Sprintf(":%s", env.Read(env.PORT)))
}
