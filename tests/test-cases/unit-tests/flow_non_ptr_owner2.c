#pragma safety enable


int socket();
void close(int fd);

int main()
{
  int fd;

  fd = socket();
  if (fd < 0)
  {
     static_set(fd, "null");
     return 1;
  }
  close(fd);
}

