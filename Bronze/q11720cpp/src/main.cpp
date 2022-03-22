#include<iostream>
using namespace std;

int main(){

    int size; int sum = 0;
    cin >> size;
    
    char temp;
    for (int i = 0; i < size; i++){
        cin >> temp;
        sum = sum + temp - '0';
    } cout << sum << endl;
}