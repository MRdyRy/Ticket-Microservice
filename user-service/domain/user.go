package domain

type User struct {
	ID       int    `json:"id"`
	Name     string `json:"name"`
	Email    string `json:"email"`
	Status   int    `json:"status"`
	Sex      string `json:"sex"`
	Password string `json:"password"`
	CreateAt string `json:"createAt"`
	CreateBy string `json:"createBy"`
	UpdateAt string `json:"updateAt"`
	UpdateBy string `json:"updateBy"`
}

type UserRepository interface {
	CreateUser(user User) (User, error)
	UpdateUser(user User) (User, error)
	DeleteUser(userId int) error
	GetUserByNameAndPassword(name string, password string) (User, error)
	GetUser(int) (User, error)
}
