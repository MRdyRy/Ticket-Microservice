package config

import (
	"log"
	"os"
	"path/filepath"

	"github.com/spf13/viper"
)

type Config struct {
	Server struct {
		Address string `mapstructure:"SERVER_ADDRESS"`
	}
	DB struct {
		Addr string `mapstructure:"DB_ADDR"`
	}
	Redis struct {
		Addr     string `mapstructure:"REDIS_ADDR"`
		Username string `mapstructure:"REDIS_USERNAME"`
		Password string `mapstructure:"REDIS_PASSWORD"`
	}
	Kafka struct {
		Addr  []string `mapstructure:"KAFKA_ADDR"`
		Topic string   `mapstructure:"KAFKA_TOPIC"`
	}
}

func LoadConfig() *Config {
	var cfg Config

	wd, err := os.Getwd()
	if err != nil {
		log.Fatal("Error getting current working directory:", err)
	}
	parentDir := filepath.Dir(wd)

	viper.AddConfigPath(parentDir)
	viper.SetConfigFile(".env")

	err = viper.ReadInConfig()
	if err != nil {
		panic(err)
	}

	err = viper.Unmarshal(&cfg)
	if err != nil {
		panic(err)
	}

	return &cfg

}
