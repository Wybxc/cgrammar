struct X { const char* text; };
void destroyX(struct X x)
{
	assert(x.text == 0);
}

int main()
{
}
