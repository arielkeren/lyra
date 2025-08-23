package main

import (
	"context"
	"log"
	"net/http"
	"os"
	"strings"
	"time"

	"github.com/joho/godotenv"
	"go.mongodb.org/mongo-driver/v2/mongo"
	"go.mongodb.org/mongo-driver/v2/mongo/options"

	authHandlers "github.com/arielkeren/lyra/handlers/auth"
	packageHandlers "github.com/arielkeren/lyra/handlers/packages"
	userHandlers "github.com/arielkeren/lyra/handlers/users"
	"github.com/arielkeren/lyra/middleware"
	"github.com/arielkeren/lyra/routing"
	"github.com/arielkeren/lyra/utils"
)

var client *mongo.Client
var db *mongo.Database

func main() {
	if err := godotenv.Load(); err != nil {
		log.Println("No .env file found")
	}

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	var err error
	clientOptions := options.Client().ApplyURI(os.Getenv("MONGO_URI"))
	client, err = mongo.Connect(clientOptions)
	if err != nil {
		log.Fatal("Failed to connect to MongoDB:", err)
	}
	defer client.Disconnect(ctx)

	if err = client.Ping(ctx, nil); err != nil {
		log.Fatal("Failed to ping MongoDB:", err)
	}

	db = client.Database("lyra")

	routing.Route("/auth/register", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == "POST" {
			authHandlers.Register(w, r, db)
		} else {
			utils.SendMethodNotAllowed(w)
		}
	})

	routing.Route("/auth/login", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == "POST" {
			authHandlers.Login(w, r, db)
		} else {
			utils.SendMethodNotAllowed(w)
		}
	})

	routing.Route("/packages", func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case "GET":
			packageHandlers.GetPackages(w, r, db)
		case "POST":
			middleware.AuthMiddleware(func(w http.ResponseWriter, r *http.Request) {
				packageHandlers.CreatePackage(w, r, db)
			})(w, r)
		default:
			utils.SendMethodNotAllowed(w)
		}
	})

	routing.Route("/packages/", func(w http.ResponseWriter, r *http.Request) {
		if strings.TrimPrefix(r.URL.Path, "/packages/") != "" {
			switch r.Method {
			case "GET":
				packageHandlers.GetPackage(w, r, db)
			case "PUT":
				middleware.AuthMiddleware(func(w http.ResponseWriter, r *http.Request) {
					packageHandlers.UpdatePackage(w, r, db)
				})(w, r)
			case "DELETE":
				middleware.AuthMiddleware(func(w http.ResponseWriter, r *http.Request) {
					packageHandlers.DeletePackage(w, r, db)
				})(w, r)
			default:
				utils.SendMethodNotAllowed(w)
			}
		} else {
			utils.SendError(w, http.StatusBadRequest, "Package name required")
		}
	})

	routing.Route("/users", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == "PUT" {
			middleware.AuthMiddleware(func(w http.ResponseWriter, r *http.Request) {
				userHandlers.UpdateUser(w, r, db)
			})(w, r)
		} else {
			utils.SendMethodNotAllowed(w)
		}
	})

	routing.Route("/users/", func(w http.ResponseWriter, r *http.Request) {
		if strings.TrimPrefix(r.URL.Path, "/users/") != "" {
			if r.Method == "GET" {
				userHandlers.GetUser(w, r, db)
			} else {
				utils.SendMethodNotAllowed(w)
			}
		} else {
			utils.SendError(w, http.StatusBadRequest, "User ID required")
		}
	})

	log.Println("Server starting on :8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
