class LinkedList {

private:
    class Node {

    public:
        int value;
        Node* next;
        Node(int value) : value(value), next(nullptr) {
        }
    };

    Node* head;
    Node* tail;
    int length;

public:
    // This is a constructor function
    LinkedList(int value) {
        Node* newNode = new Node(value);
        head = newNode;
        tail = newNode;
        length = 1;
    }

    // NOTE: `~` means that is is a destructor function

    // This is a function that gets called when the Linked List is deleted

    // The reason why you have to write a destructor function for the 
    // LinkedList is because the elements in the LinkedList are created
    // by the Node class. 
    // So if you don't create a custom destructor function,
    // when you use the line:
    // `delete linkedListOne;`
    // only the head, tail, and length variable of this class 
    // will be deleted.

    // The Nodes (the elements created by the Node class) will stay in memory. 
    ~LinkedList() {
        Node* temp = head;
        while (temp != nullptr) {
            // Move the `head` pointer to the next node,
            // then delete temp (which will be pointing to the previous
            // node that will be deleted)
            head = head->next;
            delete temp;

            // Then update the position of temp.
            temp = head;
        }
    }
};

int main() {
    LinkedList* linkedListOne = new LinkedList(4);

    // Delete will call the destructor function on the class.
    delete linkedListOne;
}
