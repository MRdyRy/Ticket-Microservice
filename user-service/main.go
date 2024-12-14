package main

import (
	"context"
	"log"
	"net/http"
	"time"
	"user-service/config"
	"user-service/handler"
	"user-service/messaging"
	"user-service/repository"
	"user-service/service"

	"github.com/go-redis/redis/v8"
	"github.com/gorilla/mux"
	"github.com/jackc/pgx/v4/pgxpool"
)

func main() {
	//load config
	cfg := config.LoadConfig()

	//init db connection
	pg_conn, err := pgxpool.Connect(context.Background(), cfg.DB.Addr)
	if err != nil {
		log.Fatalf("Unable to connect database: %v\n", err)
	}
	defer pg_conn.Close()
	dbRepository := repository.NewPostgres(pg_conn)

	redis := redis.NewClient(&redis.Options{
		Addr:     cfg.Redis.Addr,
		Username: cfg.Redis.Username,
		Password: cfg.Redis.Password,
	})

	redisRepository := repository.NewRedisCache(redis)

	kafkaRepository := messaging.NewKafkaProducer(cfg.Kafka.Addr, cfg.Kafka.Topic)

	userService := service.NewUserService(dbRepository, redisRepository, kafkaRepository)

	userHandler := handler.NewUserHandler(userService)

	r := mux.NewRouter()
	r.HandleFunc("/", userHandler.Healthcheck).Methods("GET")
	r.HandleFunc("/api/v1/user", userHandler.CreateUser).Methods("POST")
	r.HandleFunc("/api/v1/update/user", userHandler.UpdateUser)
	r.HandleFunc("/api/v1/user/{id}", userHandler.DeleteUser).Methods("DELETE")
	r.HandleFunc("/api/v1/validate", userHandler.ValidateUser).Methods("POST")
	r.HandleFunc("/api/v1/user/{id}", userHandler.GetUser).Methods("GET")

	server := &http.Server{
		Addr:         cfg.Server.Address,
		Handler:      r,
		WriteTimeout: 30 * time.Second,
		ReadTimeout:  30 * time.Second,
	}

	log.Fatal(server.ListenAndServe())
	log.Printf("Apps Running on : %v", cfg.Server.Address)
}
