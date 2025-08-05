#pragma safety enable

struct X {
  char *name;
};

struct X * make();
void del(struct X * p);

int main() {
   struct X * p = make();
   p++;
#pragma cake diagnostic check "-E1310"

   p--;
#pragma cake diagnostic check "-E1320"

   del(p);
}


