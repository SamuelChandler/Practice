//promps user to add classes until they select calculate and a 
#include <vector>
#include <iostream>
#include <string>
using namespace std;

class Course{
    public:
        double GPA;
        string name; 
        int credit_hours;

        Course(){
            GPA = 0;
            name = "None";
            credit_hours = 0;
        }

        Course(string b, double a,  int c){
            GPA = a;
            name = b;
            credit_hours = c;
        }

        void print(){
            cout << name << " " << to_string(credit_hours) << " " << to_string(GPA) << endl;
        }

        friend istream& operator>>(istream &is, Course &C){
            is >> C.name >> C.credit_hours >> C.GPA ;
            return is;
        }

};

int main(){

    
    int class_number;
    double GPA_total = 0;
    int hour_total = 0;

    cout << "Please Enter the Number of Classes you have taken: ";
    cin >> class_number;
    vector <Course> classes(class_number);

    for (int i = 0; i < class_number; i++)
    {
        cout << "please enter a class in the following format, name credit_hours GPA" << endl;
        cin >> classes[i];
        GPA_total += classes[i].GPA * classes[i].credit_hours;
        hour_total += classes[i].credit_hours;
    }

    cout << endl << "Classes:" << endl;

    for(auto &ele : classes){
        ele.print();
    }

    cout << endl << "Total Hours: " << hour_total << endl;
    cout << "Cumulitive GPA: " << GPA_total/hour_total << endl;

    return 0;
}