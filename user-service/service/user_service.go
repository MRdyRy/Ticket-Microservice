package service

import (
	"user-service/domain"
	"user-service/messaging"
	"user-service/repository"
)

type userService struct {
	repo     domain.UserRepository
	cache    *repository.RedisCache
	producer *messaging.KafkaProducer
}

type UserService interface {
	CreateUser(domain.User) (*domain.User, error)
	UpdateUser(domain.User) (*domain.User, error)
	DeleteUser(int) error
	GetUserByNameAndPassword(string, string) (*domain.User, error)
	GetUser(int) (*domain.User, error)
}

func NewUserService(repo domain.UserRepository, cache *repository.RedisCache, producer *messaging.KafkaProducer) *userService {
	return &userService{
		repo:     repo,
		cache:    cache,
		producer: producer,
	}
}

func (s *userService) CreateUser(user domain.User) (*domain.User, error) {
	var userDB domain.User

	userDB, err := s.repo.CreateUser(user)
	if err != nil {
		return nil, err
	}
	return &userDB, nil
}

func (s *userService) UpdateUser(user domain.User) (*domain.User, error) {
	var userDB domain.User
	userDB, err := s.repo.UpdateUser(user)
	if err != nil {
		return nil, err
	}
	return &userDB, nil
}

func (s *userService) DeleteUser(userId int) error {
	err := s.repo.DeleteUser(userId)
	if err != nil {
		return err
	}
	return nil
}

func (s *userService) GetUserByNameAndPassword(name, password string) (*domain.User, error) {
	user, err := s.repo.GetUserByNameAndPassword(name, password)
	if err != nil {
		return nil, err
	}

	return &user, nil
}

func (s *userService) GetUser(userId int) (*domain.User, error) {
	user, err := s.repo.GetUser(userId)
	if err != nil {
		return nil, err
	}

	return &user, nil
}
