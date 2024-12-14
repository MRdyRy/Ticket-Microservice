package repository

import (
	"context"
	"log"
	"time"

	"github.com/go-redis/redis/v8"
)

type RedisCache struct {
	client *redis.Client
}

func NewRedisCache(client *redis.Client) *RedisCache {
	if err := client.Ping(context.Background()).Err(); err != nil {
		log.Fatalf("Unable to connect redis server : %v\n", err)
	}
	return &RedisCache{
		client: client,
	}
}

func (r *RedisCache) Set(ctx context.Context, key string, value interface{}, exp time.Duration) error {
	return r.client.Set(ctx, key, value, exp).Err()
}

func (r *RedisCache) Get(ctx context.Context, key string) (string, error) {
	return r.client.Get(ctx, key).Result()
}

func (r *RedisCache) Del(ctx context.Context, key string) error {
	return r.client.Del(ctx, key).Err()
}
